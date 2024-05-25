use tonic::{transport::Server, Request, Response, Status};
use crate::network::proto::peer_node_communication::{MessageRequest, MessageReply, peer_node_communication_server::{PeerNodeCommunication, PeerNodeCommunicationServer}};
use chrono::Utc;

pub mod proto {
    tonic::include_proto!("generated/peer_node_communication");
}

#[derive(Debug, Default)]
pub struct MyPeerNodeCommunication {}

#[tonic::async_trait]
impl PeerNodeCommunication for MyPeerNodeCommunication {
    async fn send_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageReply>, Status> {
        let req = request.into_inner();

        // 处理消息
        println!("Received message: {}", req.message);
        println!("From: {}", req.sender_id);
        println!("To: {}", req.recipient_id);
        println!("Timestamp: {}", req.timestamp);

        // 生成响应
        let reply = MessageReply {
            response: format!("Received your message: {}", req.message),
            received_timestamp: Utc::now().timestamp_millis(),
            success: true,
            error_message: String::new(),
        };

        Ok(Response::new(reply))
    }
}

pub async fn start_peer_node_communication_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let peer_node_communication = MyPeerNodeCommunication::default();

    Server::builder()
        .add_service(PeerNodeCommunicationServer::new(peer_node_communication))
        .serve(addr)
        .await?;

    Ok(())
}
