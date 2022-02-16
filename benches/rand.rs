use criterion::{criterion_group, criterion_main, Criterion};

use bytes::BufMut;

use wyrng::WyRng;

fn bench_wyrand(rng: &mut WyRng) {
    let mut rand = [0_u8; 1528];
    for i in 0..191 {
        let r = rng.generate();
        rand[8 * i + 0] = 0xFF & (r >>  0) as u8;
        rand[8 * i + 1] = 0xFF & (r >>  8) as u8;
        rand[8 * i + 2] = 0xFF & (r >> 16) as u8;
        rand[8 * i + 3] = 0xFF & (r >> 24) as u8;
        rand[8 * i + 4] = 0xFF & (r >> 32) as u8;
        rand[8 * i + 5] = 0xFF & (r >> 40) as u8;
        rand[8 * i + 6] = 0xFF & (r >> 48) as u8;
        rand[8 * i + 7] = 0xFF & (r >> 56) as u8;
    }
}

fn bench_wyrand_with_offset(rng: &mut WyRng) {
    let mut rand = [0_u8; 1528];
    let mut offset = 0;
    for _ in 0..191 {
        let r = rng.generate();
        rand[offset + 0] = 0xFF & (r >>  0) as u8;
        rand[offset + 1] = 0xFF & (r >>  8) as u8;
        rand[offset + 2] = 0xFF & (r >> 16) as u8;
        rand[offset + 3] = 0xFF & (r >> 24) as u8;
        rand[offset + 4] = 0xFF & (r >> 32) as u8;
        rand[offset + 5] = 0xFF & (r >> 40) as u8;
        rand[offset + 6] = 0xFF & (r >> 48) as u8;
        rand[offset + 7] = 0xFF & (r >> 56) as u8;
        offset += 8;
    }
}

fn bench_wyrand_with_offset_with_vec_slice_ne(rng: &mut WyRng) {
    let mut buf = Vec::<u8>::with_capacity(1528);
    for _ in 0..191 {
        buf.put_slice(&rng.generate().to_ne_bytes()[..]);
    }
}

fn bench_wyrand_with_offset_with_vec_slice_be(rng: &mut WyRng) {
    let mut buf = Vec::<u8>::with_capacity(1528);
    for _ in 0..191 {
        buf.put_slice(&rng.generate().to_be_bytes()[..]);
    }
}

fn bench_wyrand_with_offset_with_vec_slice_le(rng: &mut WyRng) {
    let mut buf = Vec::<u8>::with_capacity(1528);
    for _ in 0..191 {
        buf.put_slice(&rng.generate().to_le_bytes()[..]);
    }
}

fn bench_wyrand_with_offset_with_vec(rng: &mut WyRng) {
    let mut buf = Vec::<u8>::with_capacity(1528);
    for _ in 0..191 {
        buf.put_u64(rng.generate());
    }
}

fn bench_wyrand_with_offset_with_vec_iter_ne(rng: &mut WyRng) {
    let mut buf = Vec::<u8>::with_capacity(1528);
    for _ in 0..191 {
        buf.extend(rng.generate().to_ne_bytes());
    }
}

fn bench_wyrand_with_offset_with_vec_iter_be(rng: &mut WyRng) {
    let mut buf = Vec::<u8>::with_capacity(1528);
    for _ in 0..191 {
        buf.extend(rng.generate().to_be_bytes());
    }
}

fn bench_wyrand_with_offset_with_vec_iter_le(rng: &mut WyRng) {
    let mut buf = Vec::<u8>::with_capacity(1528);
    for _ in 0..191 {
        buf.extend(rng.generate().to_le_bytes());
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = wyrng::WyRng::new(3);

    c.bench_function("wyrand", |b| b.iter(|| bench_wyrand(&mut rng)));

    c.bench_function("wyrand with offset", |b| b.iter(|| bench_wyrand_with_offset(&mut rng)));

    c.bench_function("wyrand with vec(put_u64)", |b| b.iter(|| bench_wyrand_with_offset_with_vec(&mut rng)));

    c.bench_function("wyrand with vec(put_slice) ne", |b| b.iter(|| bench_wyrand_with_offset_with_vec_slice_ne(&mut rng)));
    c.bench_function("wyrand with vec(put_slice) be", |b| b.iter(|| bench_wyrand_with_offset_with_vec_slice_be(&mut rng)));
    c.bench_function("wyrand with vec(put_slice) le", |b| b.iter(|| bench_wyrand_with_offset_with_vec_slice_le(&mut rng)));

    c.bench_function("wyrand with vec extend iter ne", |b| b.iter(|| bench_wyrand_with_offset_with_vec_iter_ne(&mut rng)));
    c.bench_function("wyrand with vec extend iter be", |b| b.iter(|| bench_wyrand_with_offset_with_vec_iter_be(&mut rng)));
    c.bench_function("wyrand with vec extend iter le", |b| b.iter(|| bench_wyrand_with_offset_with_vec_iter_le(&mut rng)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
