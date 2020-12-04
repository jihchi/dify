use criterion::{criterion_group, criterion_main, Criterion};
use dify::diff;
use image::{io::Reader as ImageReader, RgbaImage};

fn get_image(path: &str) -> RgbaImage {
    ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba8()
}

fn criterion_benchmark(c: &mut Criterion) {
    let default_run_params = diff::RunParams {
        left: "",
        right: "",
        output: "",
        threshold: 0.1,
        output_image_base: None,
        do_not_check_dimensions: true,
        detect_anti_aliased_pixels: false,
        blend_factor_of_unchanged_pixels: None,
    };

    c.bench_function("1000 × 667 pixels", |b| {
        let left_image = get_image("./benches/fixtures/tiger.jpg");
        let right_image = get_image("./benches/fixtures/tiger-2.jpg");

        b.iter(|| diff::get_results(&left_image, &right_image, &default_run_params))
    });

    c.bench_function("8400 × 4725 pixels", |b| {
        let left_image = get_image("./benches/fixtures/water-4k.png");
        let right_image = get_image("./benches/fixtures/water-4k-2.png");

        b.iter(|| diff::get_results(&left_image, &right_image, &default_run_params))
    });

    c.bench_function("3446 × 10728 pixels", |b| {
        let left_image = get_image("./benches/fixtures/www.cypress.io.png");
        let right_image = get_image("./benches/fixtures/www.cypress.io-2.png");

        b.iter(|| diff::get_results(&left_image, &right_image, &default_run_params))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}

criterion_main!(benches);
