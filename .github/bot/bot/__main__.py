# Copyright (C) 2026 meta-magic_mount-rs developers
# SPDX-License-Identifier: GPL-v3

from logging import basicConfig
from asyncio import run
from . import main

basicConfig(
    level="INFO",
    format="%(levelname)s - %(message)s",
)

run(main())
