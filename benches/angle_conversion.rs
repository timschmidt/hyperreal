use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hyperreal::{Rational, Real};

#[path = "support/bench_docs.rs"]
mod bench_docs;

use bench_docs::{BenchDoc, BenchGroupDoc};

const ANGLE_CONVERSION_GROUPS: &[BenchGroupDoc] = &[BenchGroupDoc {
    name: "angle_conversion",
    description: "Cost of `Real::to_degrees` and `Real::to_radians` across common symbolic classes.",
    benches: &[
        BenchDoc {
            name: "to_degrees_pi",
            description: "Converts pi radians to degrees (180). Class::Pi input — pi-cancellation fast path candidate.",
        },
        BenchDoc {
            name: "to_degrees_tau",
            description: "Converts 2*pi radians to degrees (360). Class::Pi input with rational scale 2.",
        },
        BenchDoc {
            name: "to_degrees_half_pi",
            description: "Converts pi/2 radians to degrees (90). Class::Pi input with fractional rational scale.",
        },
        BenchDoc {
            name: "to_degrees_rational_one",
            description: "Converts 1 radian to degrees (180/pi). Class::One input — no pi cancellation.",
        },
        BenchDoc {
            name: "to_radians_180",
            description: "Converts 180 degrees to radians (pi). Class::One input — pi-attach fast path candidate.",
        },
        BenchDoc {
            name: "to_radians_360",
            description: "Converts 360 degrees to radians (2*pi). Class::One input with larger rational.",
        },
        BenchDoc {
            name: "to_radians_90",
            description: "Converts 90 degrees to radians (pi/2). Class::One input with non-multiple scale.",
        },
        BenchDoc {
            name: "to_radians_pi",
            description: "Converts pi degrees to radians (pi^2/180). Class::Pi input — uncommon path.",
        },
        BenchDoc {
            name: "to_degrees_negative_one",
            description: "Converts -1 radian to degrees. Class::One negative — fallback path.",
        },
        BenchDoc {
            name: "to_degrees_sqrt_two",
            description: "Converts sqrt(2) radians to degrees. Class::Sqrt — fully generic dispatch.",
        },
        BenchDoc {
            name: "to_degrees_e",
            description: "Converts e radians to degrees. Class::Exp(1) — fully generic dispatch.",
        },
        BenchDoc {
            name: "to_radians_negative_pi",
            description: "Converts -pi degrees to radians. Class::Pi negative — fallback path.",
        },
        BenchDoc {
            name: "to_radians_sqrt_two",
            description: "Converts sqrt(2) degrees to radians. Class::Sqrt — fully generic dispatch.",
        },
        BenchDoc {
            name: "to_radians_e",
            description: "Converts e degrees to radians. Class::Exp(1) — fully generic dispatch.",
        },
    ],
}];

fn bench_angle_conversion(c: &mut Criterion) {
    bench_docs::write_benchmark_docs(
        "angle_conversion",
        "Microbenchmarks for radian/degree conversion across symbolic classes.",
        ANGLE_CONVERSION_GROUPS,
    );

    let mut group = c.benchmark_group("angle_conversion");

    let pi = Real::pi();
    let tau = Real::tau();
    let half_pi = Real::pi() * Real::new(Rational::fraction(1, 2).unwrap());
    let one = Real::from(1_i32);
    let n180 = Real::from(180_i32);
    let n360 = Real::from(360_i32);
    let n90 = Real::from(90_i32);
    let neg_one = Real::from(-1_i32);
    let neg_pi = -Real::pi();
    let sqrt_two = Real::from(2_i32).sqrt().unwrap();
    let e = Real::e();

    group.bench_function("to_degrees_pi", |b| {
        b.iter(|| black_box(pi.clone()).to_degrees())
    });
    group.bench_function("to_degrees_tau", |b| {
        b.iter(|| black_box(tau.clone()).to_degrees())
    });
    group.bench_function("to_degrees_half_pi", |b| {
        b.iter(|| black_box(half_pi.clone()).to_degrees())
    });
    group.bench_function("to_degrees_rational_one", |b| {
        b.iter(|| black_box(one.clone()).to_degrees())
    });
    group.bench_function("to_radians_180", |b| {
        b.iter(|| black_box(n180.clone()).to_radians())
    });
    group.bench_function("to_radians_360", |b| {
        b.iter(|| black_box(n360.clone()).to_radians())
    });
    group.bench_function("to_radians_90", |b| {
        b.iter(|| black_box(n90.clone()).to_radians())
    });
    group.bench_function("to_radians_pi", |b| {
        b.iter(|| black_box(pi.clone()).to_radians())
    });
    group.bench_function("to_degrees_negative_one", |b| {
        b.iter(|| black_box(neg_one.clone()).to_degrees())
    });
    group.bench_function("to_degrees_sqrt_two", |b| {
        b.iter(|| black_box(sqrt_two.clone()).to_degrees())
    });
    group.bench_function("to_degrees_e", |b| {
        b.iter(|| black_box(e.clone()).to_degrees())
    });
    group.bench_function("to_radians_negative_pi", |b| {
        b.iter(|| black_box(neg_pi.clone()).to_radians())
    });
    group.bench_function("to_radians_sqrt_two", |b| {
        b.iter(|| black_box(sqrt_two.clone()).to_radians())
    });
    group.bench_function("to_radians_e", |b| {
        b.iter(|| black_box(e.clone()).to_radians())
    });

    group.finish();
}

criterion_group!(benches, bench_angle_conversion);
criterion_main!(benches);
