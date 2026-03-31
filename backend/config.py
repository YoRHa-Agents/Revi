import os
from pathlib import Path
from dataclasses import dataclass, field


@dataclass
class Settings:
    workspace_path: Path = field(default_factory=lambda:
        Path(os.environ["REVI_WORKSPACE"]) if "REVI_WORKSPACE" in os.environ
        else Path(__file__).parent / "workspace")
    data_path: Path = field(default_factory=lambda:
        Path(os.environ["REVI_DATA"]) if "REVI_DATA" in os.environ
        else Path(__file__).parent / "data")
    allowed_origins: list = field(default_factory=lambda: [
        "http://localhost:5173",
        "http://localhost:5174",
        "http://localhost:5175",
        "http://localhost:5176",
    ])

    @property
    def uploads_path(self): return self.workspace_path


settings = Settings()
