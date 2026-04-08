import importlib
import unittest


class PackageImportTest(unittest.TestCase):
    def test_package_imports(self) -> None:
        module = importlib.import_module("research_agent")
        self.assertEqual(module.__all__, ["__version__"])
