# 重命名样例基准集

> 阶段 1：需求规格定义
> 项目：media-renamer-Kivo
> 日期：2026-06-08
> 样例总数：45 条

---

## 格式说明

每条样例包含：输入文件名、输入路径、推断类型、预期解析结果、预期新名称、是否跳过、是否人工确认、风险说明。

---

## 一、电影（8 条）

| ID | 输入文件名 | 推断类型 | 预期新名称 | 跳过 | 人工确认 |
|----|-----------|---------|-----------|:---:|:-------:|
| M-001 | `The.Shawshank.Redemption.1994.1080p.BluRay.x264-SPARKS.mkv` | 电影 | `The Shawshank Redemption (1994) [1080p BluRay x264].mkv` | 否 | 否 |
| M-002 | `Inception.1080p.BluRay.x264.mkv` | 电影(低) | `Inception [1080p BluRay x264].mkv` | 否 | 是(缺年份) |
| M-003 | `霸王别姬.1993.1080p.BluRay.x265-CHD.mkv` | 电影 | `霸王别姬 (1993) [1080p BluRay x265].mkv` | 否 | 否 |
| M-004 | `Blade.Runner.2049.2017.2160p.UHD.BluRay.REMUX.DV.HDR.HEVC.TrueHD.Atmos.7.1-FGT.mkv` | 电影 | `Blade Runner 2049 (2017) [2160p BluRay REMUX HEVC TrueHD Atmos].mkv` | 否 | 否 |
| M-005 | `Dune.2021.1080p.WEB-DL.DD5.1.H.264.mkv` | 电影 | `Dune (2021) [1080p WEB-DL H.264 DD5.1].mkv` | 否 | 否 |
| M-006 | `Spider-Man.No.Way.Home.2021.1080p.BluRay.x264-SPARKS.mkv` | 电影 | `Spider-Man No Way Home (2021) [1080p BluRay x264].mkv` | 否 | 否 |
| M-007 | `2001.A.Space.Odyssey.1968.1080p.BluRay.x264.mkv` | 电影 | `2001 A Space Odyssey (1968) [1080p BluRay x264].mkv` | 否 | 否 |
| M-008 | `The.Godfather.1972.DVD.XviD-LKRG.avi` | 电影 | `The Godfather (1972) [DVD XviD].avi` | 否 | 否 |

**解析详情**：
- M-001: Title="The Shawshank Redemption", Year=1994, Resolution=1080p, Source=BluRay, VideoCodec=x264, Group=SPARKS
- M-002: Title="Inception", Year=null, Resolution=1080p, Source=BluRay, VideoCodec=x264 — **风险**: 缺少年份
- M-003: Title="霸王别姬", Year=1993, Resolution=1080p, Source=BluRay, VideoCodec=x265, Group=CHD
- M-004: Title="Blade Runner 2049", Year=2017, Resolution=2160p, Source=BluRay REMUX, VideoCodec=HEVC, AudioCodec=TrueHD Atmos — **风险**: 标题含数字
- M-005: Title="Dune", Year=2021, Resolution=1080p, Source=WEB-DL, VideoCodec=H.264, AudioCodec=DD5.1
- M-006: Title="Spider-Man No Way Home", Year=2021, Resolution=1080p, Source=BluRay, VideoCodec=x264
- M-007: Title="2001 A Space Odyssey", Year=1968, Resolution=1080p, Source=BluRay, VideoCodec=x264 — **风险**: 标题含数字
- M-008: Title="The Godfather", Year=1972, Source=DVD, VideoCodec=XviD, Group=LKRG

---

## 二、欧美剧（8 条）

| ID | 输入文件名 | 推断类型 | 预期新名称 | 跳过 | 人工确认 |
|----|-----------|---------|-----------|:---:|:-------:|
| S-001 | `Breaking.Bad.S01E01.720p.BluRay.x264-REWARD.mkv` | 电视剧 | `Breaking Bad - S01E01.mkv` | 否 | 否 |
| S-002 | `Game.of.Thrones.S01E01-E02.1080p.BluRay.x264-ROVERS.mkv` | 电视剧 | `Game of Thrones - S01E01-E02.mkv` | 否 | 否 |
| S-003 | `Friends.S01E01.The.One.Where.Monica.Gets.a.Roommate.720p.BluRay.mkv` | 电视剧 | `Friends - S01E01 - The One Where Monica Gets a Roommate.mkv` | 否 | 否 |
| S-004 | `Stranger.Things.S02E01.1080p.NF.WEB-DL.mkv` | 电视剧 | `Stranger Things - S02E01.mkv` | 否 | 否 |
| S-005 | `The.Office.US.S01E01.480p.DVD.mkv` | 电视剧 | `The Office US - S01E01.mkv` | 否 | 否 |
| S-006 | `The.Walking.Dead.S10E01.HDTV.x264-SVA.mkv` | 电视剧 | `The Walking Dead - S10E01.mkv` | 否 | 否 |
| S-007 | `Seinfeld.01.mkv` | 电视剧(低) | `Seinfeld - 01.mkv` | 否 | 是(缺季号) |
| S-008 | `Succession.S04E01.1080p.WEBRip.x265-KRAKEN.mkv` | 电视剧 | `Succession - S04E01.mkv` | 否 | 否 |

**解析详情**：
- S-001: SeriesTitle="Breaking Bad", Season=1, Episode=1, Resolution=720p, Source=BluRay, VideoCodec=x264
- S-002: SeriesTitle="Game of Thrones", Season=1, EpisodeStart=1, EpisodeEnd=2 — **多集格式保留**
- S-003: SeriesTitle="Friends", Season=1, Episode=1, EpisodeTitle="The One Where Monica Gets a Roommate"
- S-004: SeriesTitle="Stranger Things", Season=2, Episode=1, Resolution=1080p, Source=WEB-DL
- S-005: SeriesTitle="The Office US", Season=1, Episode=1, Resolution=480p, Source=DVD
- S-006: SeriesTitle="The Walking Dead", Season=10, Episode=1, Source=HDTV, VideoCodec=x264
- S-007: SeriesTitle="Seinfeld", Episode=1 — **风险**: 缺少年份和季号，低置信度
- S-008: SeriesTitle="Succession", Season=4, Episode=1, Resolution=1080p, Source=WEBRip, VideoCodec=x265

---

## 三、日剧/韩剧/国产剧（6 条）

| ID | 输入文件名 | 推断类型 | 预期新名称 | 跳过 | 人工确认 |
|----|-----------|---------|-----------|:---:|:-------:|
| A-001 | `半泽直树.S01E01.1080p.WEB-DL.mkv` | 电视剧 | `半泽直树 - S01E01.mkv` | 否 | 否 |
| A-002 | `鱿鱼游戏.S01E01.1080p.NF.WEB-DL.mkv` | 电视剧 | `鱿鱼游戏 - S01E01.mkv` | 否 | 否 |
| A-003 | `琅琊榜.S01E01.1080p.WEB-DL.H264.mkv` | 电视剧 | `琅琊榜 - S01E01.mkv` | 否 | 否 |
| A-004 | `孤独的美食家.E01.1080p.WEB-DL.mkv` | 电视剧 | `孤独的美食家 - S01E01.mkv` | 否 | 否 |
| A-005 | `Signal시그널.S02E01.1080p.WEB-DL.mkv` | 电视剧 | `Signal시그널 - S02E01.mkv` | 否 | 否 |
| A-006 | `庆余年.第二季.E01.1080p.WEB-DL.mkv` | 电视剧 | `庆余年 - S02E01.mkv` | 否 | 否 |

**解析详情**：
- A-001: SeriesTitle="半泽直树", Season=1, Episode=1, Resolution=1080p, Source=WEB-DL
- A-002: SeriesTitle="鱿鱼游戏", Season=1, Episode=1, Resolution=1080p, Source=WEB-DL
- A-003: SeriesTitle="琅琊榜", Season=1, Episode=1, Resolution=1080p, Source=WEB-DL, VideoCodec=H264
- A-004: SeriesTitle="孤独的美食家", Season=1, Episode=1 — **无季号默认S01**
- A-005: SeriesTitle="Signal시그널", Season=2, Episode=1 — **混合语言标题**
- A-006: SeriesTitle="庆余年", Season=2, Episode=1 — **中文季号"第二季"**

---

## 四、动漫（10 条）

| ID | 输入文件名 | 推断类型 | 预期新名称 | 跳过 | 人工确认 |
|----|-----------|---------|-----------|:---:|:-------:|
| AN-001 | `[SubsPlease] One Piece - 1080 [E3F21422].mkv` | 动漫 | `One Piece - S01E1080 [SubsPlease][1080p].mkv` | 否 | 否 |
| AN-002 | `[喵萌奶茶屋] 间谍过家家 - 01 [1080p][简日双语].mkv` | 动漫 | `间谍过家家 - S01E01 [喵萌奶茶屋][1080p].mkv` | 否 | 否 |
| AN-003 | `[SubsPlease] Shingeki no Kyojin - S04E28 [1080p].mkv` | 动漫 | `Shingeki no Kyojin - S04E28 [SubsPlease][1080p].mkv` | 否 | 否 |
| AN-004 | `进击的巨人.S04E28.1080p.mkv` | 动漫(低) | `进击的巨人 - S04E28 [1080p].mkv` | 否 | 是(无组名) |
| AN-005 | `[FFF] Naruto Shippuuden - 220 [BD][1080p][FLAC][D1534628].mkv` | 动漫 | `Naruto Shippuuden - S01E220 [FFF][1080p].mkv` | 否 | 否 |
| AN-006 | `[SubsPlease] Boku no Hero Academia 6 - 01 [1080p].mkv` | 动漫 | `Boku no Hero Academia - S06E01 [SubsPlease][1080p].mkv` | 否 | 否 |
| AN-007 | `[ANi] SPY×FAMILY - 01 [1080p][Baha][WEB-DL][AAC AVC][CHT].mp4` | 动漫 | `SPY×FAMILY - S01E01 [ANi][1080p].mp4` | 否 | 否 |
| AN-008 | `[SubsPlease] One Piece - 1079-1080 [1080p].mkv` | 动漫 | `One Piece - S01E1079-E1080 [SubsPlease][1080p].mkv` | 否 | 否 |
| AN-009 | `[Erai-raws] Detective Conan - 1100 [480p][Multiple Subtitle].mkv` | 动漫 | `Detective Conan - S01E1100 [Erai-raws][480p].mkv` | 否 | 否 |
| AN-010 | `[千夏字幕组][名侦探柯南][1100][1080p][MP4].mp4` | 动漫 | `名侦探柯南 - S01E1100 [千夏字幕组][1080p].mp4` | 否 | 否 |

**解析详情**：
- AN-001: Group="SubsPlease", SeriesTitle="One Piece", Episode=1080, Resolution=1080p
- AN-002: Group="喵萌奶茶屋", SeriesTitle="间谍过家家", Episode=1, Resolution=1080p
- AN-003: Group="SubsPlease", SeriesTitle="Shingeki no Kyojin", Season=4, Episode=28
- AN-004: SeriesTitle="进击的巨人", Season=4, Episode=28 — **风险**: 无组名，动漫/剧集边界模糊
- AN-005: Group="FFF", SeriesTitle="Naruto Shippuuden", Episode=220, Source=BD, Resolution=1080p
- AN-006: Group="SubsPlease", SeriesTitle="Boku no Hero Academia", Season=6, Episode=1 — **标题含数字6**
- AN-007: Group="ANi", SeriesTitle="SPY×FAMILY", Episode=1, Resolution=1080p, Source=WEB-DL
- AN-008: Group="SubsPlease", SeriesTitle="One Piece", EpisodeStart=1079, EpisodeEnd=1080 — **多集**
- AN-009: Group="Erai-raws", SeriesTitle="Detective Conan", Episode=1100, Resolution=480p
- AN-010: Group="千夏字幕组", SeriesTitle="名侦探柯南", Episode=1100, Resolution=1080p

---

## 五、特别篇/SP/OVA/NCOP/NCED/Extras（6 条）

| ID | 输入文件名 | 推断类型 | 预期新名称 | 跳过 | 人工确认 |
|----|-----------|---------|-----------|:---:|:-------:|
| SP-001 | `Breaking.Bad.El.Camino.2019.1080p.NF.WEB-DL.mkv` | 特别篇 | `Breaking Bad - S00E01 - El Camino.mkv` | 否 | 否 |
| SP-002 | `半泽直树.SP.2014.720p.WEB-DL.mkv` | 特别篇 | `半泽直树 - S00E01 - SP.mkv` | 否 | 否 |
| SP-003 | `进击的巨人.OVA.01.1080p.BluRay.mkv` | OVA | `进击的巨人 - S00E01.mkv` | 否 | 否 |
| SP-004 | `[SubsPlease] Shingeki no Kyojin - NCOP01 [1080p].mkv` | NCOP | `Shingeki no Kyojin - Extra - NCOP01.mkv` | 否 | 否 |
| SP-005 | `[SubsPlease] Shingeki no Kyojin - NCED01 [1080p].mkv` | NCED | `Shingeki no Kyojin - Extra - NCED01.mkv` | 否 | 否 |
| SP-006 | `Friends.Extras.S01.Behind.The.Scenes.mkv` | Extras | `Friends - Extra - Extras01.mkv` | 否 | 否 |

---

## 六、低置信度/冲突/异常（7 条）

| ID | 输入文件名 | 推断类型 | 预期新名称 | 跳过 | 人工确认 |
|----|-----------|---------|-----------|:---:|:-------:|
| E-001 | `video001.mkv` | 未识别(低) | (跳过) | 是 | 是 |
| E-002 | 超长路径电影 | 电影(路径过长) | (标记路径过长) | 是 | 是 |
| E-003 | `Movie: The <Story>.2024.1080p.mkv` | 电影(非法字符) | `Movie - The Story (2024) [1080p].mkv` | 否 | 是 |
| E-004 | 同目录两文件指向同一目标 | 电影(冲突) | (冲突阻断) | 是 | 是 |
| E-005 | 目标路径已存在文件 | 电视剧(冲突) | (冲突阻断) | 是 | 是 |
| E-006 | `test.mkv` | 未识别(低) | (跳过) | 是 | 是 |
| E-007 | 源文件名含特殊Unicode字符 | 电影(低) | (需人工确认) | 否 | 是 |

**详情**：
- E-001: 无法提取任何有效信息，置信度 <30
- E-002: 目标路径超过 260 字符
- E-003: 标题含 `:` 和 `<>` 非法字符，需清洗
- E-004: 同一任务内两个源文件生成相同目标路径
- E-005: 目标路径已存在同名文件
- E-006: 文件名过短，无法解析
- E-007: 文件名含特殊 Unicode 字符，解析异常

---

## 统计

| 类别 | 数量 |
|------|:----:|
| 电影 | 8 |
| 欧美剧 | 8 |
| 日剧/韩剧/国产剧 | 6 |
| 动漫 | 10 |
| 特别篇/SP/OVA/NCOP/NCED/Extras | 6 |
| 低置信度/冲突/异常 | 7 |
| **总计** | **45** |

---

*下一阶段：阶段 2 — 技术选型审查*
