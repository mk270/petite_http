//! Common `Content-Type` values, adapted from
//! [Mozilla's table](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types/Common_types)

/// AAC audio
pub const AAC: &'static [u8] = b"audio/aac";
/// AbiWord document
pub const ABW: &'static [u8] = b"application/x-abiword";
/// Animated Portable Network Graphics (APNG) image
pub const APNG: &'static [u8] = b"image/apng";
/// Archive document (multiple files embedded)
pub const ARC: &'static [u8] = b"application/x-freearc";
/// AVIF image
pub const AVIF: &'static [u8] = b"image/avif";
/// AVI: Audio Video Interleave
pub const AVI: &'static [u8] = b"video/x-msvideo";
/// Amazon Kindle eBook format
pub const AZW: &'static [u8] = b"application/vnd.amazon.ebook";
/// Any kind of binary data
pub const BIN: &'static [u8] = b"application/octet-stream";
/// Windows OS/2 Bitmap Graphics
pub const BMP: &'static [u8] = b"image/bmp";
/// BZip archive
pub const BZ: &'static [u8] = b"application/x-bzip";
/// BZip2 archive
pub const BZ2: &'static [u8] = b"application/x-bzip2";
/// CD audio
pub const CDA: &'static [u8] = b"application/x-cdf";
/// C-Shell script
pub const CSH: &'static [u8] = b"application/x-csh";
/// Cascading Style Sheets (CSS)
pub const CSS: &'static [u8] = b"text/css";
/// Comma-separated values (CSV)
pub const CSV: &'static [u8] = b"text/csv";
/// Microsoft Word
pub const DOC: &'static [u8] = b"application/msword";
/// Microsoft Word (OpenXML)
pub const DOCX: &'static [u8] = b"application/vnd.openxmlformats-officedocument.wordprocessingml.document";
/// MS Embedded OpenType fonts
pub const EOT: &'static [u8] = b"application/vnd.ms-fontobject";
/// Electronic publication (EPUB)
pub const EPUB: &'static [u8] = b"application/epub+zip";
/// GZip Compressed Archive
pub const GZ: &'static [u8] = b"application/gzip";
/// Graphics Interchange Format (GIF)
pub const GIF: &'static [u8] = b"image/gif";
/// HyperText Markup Language (HTML)
pub const HTML: &'static [u8] = b"text/html";
/// Icon format
pub const ICO: &'static [u8] = b"image/vnd.microsoft.icon";
/// iCalendar format
pub const ICS: &'static [u8] = b"text/calendar";
/// Java Archive (JAR)
pub const JAR: &'static [u8] = b"application/java-archive";
/// JPEG images
pub const JPEG: &'static [u8] = b"image/jpeg";
/// JavaScript
pub const JS: &'static [u8] = b"text/javascript";
/// JSON format
pub const JSON: &'static [u8] = b"application/json";
/// JSON-LD format
pub const JSONLD: &'static [u8] = b"application/ld+json";
/// Markdown
pub const MD: &'static [u8] = b"text/markdown";
/// Musical Instrument Digital Interface (MIDI)
pub const MIDI: &'static [u8] = b"audio/midi";
/// JavaScript module
pub const MJS: &'static [u8] = b"text/javascript";
/// MP3 audio
pub const MP3: &'static [u8] = b"audio/mpeg";
/// MP4 video
pub const MP4: &'static [u8] = b"video/mp4";
/// MPEG Video
pub const MPEG: &'static [u8] = b"video/mpeg";
/// Apple Installer Package
pub const MPKG: &'static [u8] = b"application/vnd.apple.installer+xml";
/// OpenDocument presentation document
pub const ODP: &'static [u8] = b"application/vnd.oasis.opendocument.presentation";
/// OpenDocument spreadsheet document
pub const ODS: &'static [u8] = b"application/vnd.oasis.opendocument.spreadsheet";
/// OpenDocument text document
pub const ODT: &'static [u8] = b"application/vnd.oasis.opendocument.text";
/// Ogg audio
pub const OGA: &'static [u8] = b"audio/ogg";
/// Ogg video
pub const OGV: &'static [u8] = b"video/ogg";
/// Ogg
pub const OGX: &'static [u8] = b"application/ogg";
/// Opus audio in Ogg container
pub const OPUS: &'static [u8] = b"audio/ogg";
/// OpenType font
pub const OTF: &'static [u8] = b"font/otf";
/// Portable Network Graphics
pub const PNG: &'static [u8] = b"image/png";
/// Adobe Portable Document Format (PDF)
pub const PDF: &'static [u8] = b"application/pdf";
/// Hypertext Preprocessor (Personal Home Page)
pub const PHP: &'static [u8] = b"application/x-httpd-php";
/// Microsoft PowerPoint
pub const PPT: &'static [u8] = b"application/vnd.ms-powerpoint";
/// Microsoft PowerPoint (OpenXML)
pub const PPTX: &'static [u8] = b"application/vnd.openxmlformats-officedocument.presentationml.presentation";
/// RAR archive
pub const RAR: &'static [u8] = b"application/vnd.rar";
/// Rich Text Format (RTF)
pub const RTF: &'static [u8] = b"application/rtf";
/// Bourne shell script
pub const SH: &'static [u8] = b"application/x-sh";
/// Scalable Vector Graphics (SVG)
pub const SVG: &'static [u8] = b"image/svg+xml";
/// Tape Archive (TAR)
pub const TAR: &'static [u8] = b"application/x-tar";
/// Tagged Image File Format (TIFF)
pub const TIFF: &'static [u8] = b"image/tiff";
/// MPEG transport stream
pub const TS: &'static [u8] = b"video/mp2t";
/// TrueType Font
pub const TTF: &'static [u8] = b"font/ttf";
/// Text, (generally ASCII or ISO 8859-n)
pub const TXT: &'static [u8] = b"text/plain";
/// Microsoft Visio
pub const VSD: &'static [u8] = b"application/vnd.visio";
/// Waveform Audio Format
pub const WAV: &'static [u8] = b"audio/wav";
/// WEBM audio
pub const WEBA: &'static [u8] = b"audio/webm";
/// WEBM video
pub const WEBM: &'static [u8] = b"video/webm";
/// Web application manifest
pub const WEBMANIFEST: &'static [u8] = b"application/manifest+json";
/// WEBP image
pub const WEBP: &'static [u8] = b"image/webp";
/// Web Open Font Format (WOFF)
pub const WOFF: &'static [u8] = b"font/woff";
/// Web Open Font Format (WOFF)
pub const WOFF2: &'static [u8] = b"font/woff2";
/// XHTML
pub const XHTML: &'static [u8] = b"application/xhtml+xml";
/// Microsoft Excel
pub const XLS: &'static [u8] = b"application/vnd.ms-excel";
/// Microsoft Excel (OpenXML)
pub const XLSX: &'static [u8] = b"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
/// XML
pub const XML: &'static [u8] = b"application/xml";
/// XUL
pub const XUL: &'static [u8] = b"application/vnd.mozilla.xul+xml";
/// ZIP archive
pub const ZIP: &'static [u8] = b"application/zip";
/// 3GPP audio container
pub const _3GP_AUDIO: &'static [u8] = b"audio/3gpp";
/// 3GPP video container
pub const _3GP_VIDEO: &'static [u8] = b"video/3gpp";
/// 3GPP2 audio container
pub const _3G2_AUDIO: &'static [u8] = b"audio/3gpp2";
/// 3GPP2 video container
pub const _3G2_VIDEO: &'static [u8] = b"video/3gpp2";
/// 7-zip archive
pub const _7Z: &'static [u8] = b"application/x-7z-compressed";
