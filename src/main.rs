use symphonia::core::{
    codecs::{DecoderOptions, CODEC_TYPE_NULL},
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
};

fn main() {
    // Get the first command line argument.
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("file path not provided");

    // Open the media source.
    let src = std::fs::File::open(&path).expect("failed to open media");
    let media_source_stream = MediaSourceStream::new(Box::new(src), Default::default());

    // Create a probe hint using the file's extension. [Optional]
    let mut hint = Hint::new();
    hint.with_extension("mp3");
    dbg!("{:?}", &hint);

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();
    dbg!("{:?}", &meta_opts);
    dbg!("{:?}", &fmt_opts);

    // Probe the media source.
    let probed = symphonia::default::get_probe()
        .format(&hint, media_source_stream, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    // Get the instantiated format reader.
    let format = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = Default::default();
    dbg!("{:?}", &dec_opts);

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;
    dbg!("{:?}", track_id);
}
