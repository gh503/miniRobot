import unittest
import sys

class TestDisk(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        print("Disk Test Module Setup")

    @classmethod
    def tearDownClass(cls):
        print("Disk Test Module Teardown")

    def test_disk_info_01(self):
        print("Running test_disk_info_01")
        # Add your test logic here

if __name__ == "__main__":
    if len(sys.argv) > 1:
        suite = unittest.TestSuite()
        for test_name in sys.argv[1:]:
            suite.addTest(TestCPU(test_name))
        unittest.TextTestRunner().run(suite)
    else:
        unittest.main()
