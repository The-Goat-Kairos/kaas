from __future__ import annotations

import argparse
from pathlib import Path


def create_directories(home: Path, *, dry_run: bool) -> None:
    return


def init(args: argparse.Namespace) -> int:
    home = Path.home()

    if args.dirs:
        create_directories(home, dry_run=args.dry_run)

    return 3


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="Kaas",
                                     description="Kairos' Awesome Automated System")
    commands = parser.add_subparsers(dest="command", required=True)

    init_parser = commands.add_parser("init", help="Prepare this computer for kaas")
    init_parser.add_argument(
        "--dirs",
        action="store_true",
        help="create ~/projects, ~/opt, ~/testing, ~/.local/bin, and ~/.config"
    )
    init_parser.add_argument(
        "--submodules",
        action="store_true",
        help="Initialize and update the submodules"
    )
    init_parser.add_argument(
        "--packages",
        action="store_true",
        help="Install the initial package set"
    )
    init_parser.add_argument(
        "--dry_run",
        action="store_true",
        help="Show actions without changing anything"
    )

    init_parser.set_defaults(handler=init)

    return parser


def main() -> None:
    parser = build_parser()
    args = parser.parse_args()
    raise SystemExit(args.handler(args))
