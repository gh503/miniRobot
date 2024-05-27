# tests/local/cpu_test.py
import unittest
import sys

class TestCPU(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        print("CPU Test Module Setup")

    @classmethod
    def tearDownClass(cls):
        print("CPU Test Module Teardown")

    def test_cpu_info_01(self):
        print("Running test_cpu_info_01")
        # Add your test logic here

    def test_cpu_info_02(self):
        print("Running test_cpu_info_02")
        # Add your test logic here

if __name__ == "__main__":
    if len(sys.argv) > 1:
        suite = unittest.TestSuite()
        for test_name in sys.argv[1:]:
            suite.addTest(TestCPU(test_name))
        unittest.TextTestRunner().run(suite)
    else:
        unittest.main()
