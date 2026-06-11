# media_info family

Purpose: provide normalized media technical fields for commercial PT naming.

Family tree:

- contracts: input/output/data contracts only.
- probe: media probing and filename fallback extraction only.

Current implementation:

- filename_media_info_parser extracts resolution/source/video/audio/release group from PT/BT-style release names.
- probe_media_info exposes stable normalized output for UI and future ffprobe integration.

Non-goals in this step:

- no UI ownership
- no rename execution
- no scraping
- no bundled ffprobe binary
