<div align="center">
  <img src="static/octonion64.jpg" alt="octonion64 logo" width="150" />
  <h1>octonion64</h1>
  <p><b>Bare-metal, mathematically exact 8D hypercomplex algebra over Z_2^64</b></p>
  <p><i>A Cycleless Open Source Release</i></p>
</div>

## The Philosophy: Why `u64` is IMBA

Look at the current ecosystem for hypercomplex math crates. Almost all of them rely on `f64` (floating-point). That means rounding errors, precision loss, and unpredictable performance overhead. 

`octonion64` throws that garbage out. We implement the **Cayley-Dickson construction** strictly over `u64` integers. 
The result? **Absolute mathematical precision without a single bit of error.** We run faster, cleaner, and with zero unexpected behaviors. 

To protect developers from shooting themselves in the foot (due to the chaotic nature of non-associative algebra division), **we explicitly removed division (`no_div`) and remainder (`no_rem`) operations**. No floating-point traps, no zero-division panics, no non-deterministic math. Just raw, hyper-optimized addition, multiplication, and bitwise logic.

## Core Features

- **Pure Exactness**: 100% mathematically exact outputs. If you input integers, you get strictly correct integer outputs without `f64` approximations.
- **`no_std` & `no_malloc`**: Zero heap allocations. Ready for bare-metal kernels and cycle-accurate hardware simulations.
- **Anti-Footgun (`no_div`, `no_rem`)**: Division in octonions is a nightmare of edge cases. We removed it. You literally cannot make a division mistake here.
- **LLVM Auto-Vectorization & SIMD**: Data is perfectly aligned as `[u64; 8]`. Because the algebraic operations are completely unrolled and branchless, the LLVM compiler automatically synthesizes and maps them directly into AVX2/AVX-512 SIMD instructions. You get extreme vector performance for free, without a single dirty `core::arch` intrinsic.
- **SEO-Optimized Name, Elegant Struct**: The crate is named `octonion64` so engineers can find it, but the core primitive is cleanly named `Octave`.

## Installation & Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
octonion64 = { version = "0.1.0" }
```

### Quickstart Example

```rust
use octonion64::Octave;

// Initialize two precise 8D hypercomplex numbers
let a = Octave::new(1, 2, 3, 4, 5, 6, 7, 8);
let b = Octave::splat(42);

// Mathematically exact multiplication (Non-commutative)
let c = a.mul(b);

// Calculate the associator to measure non-associativity
// [a, b, c] = (a * b) * c - a * (b * c)
let assoc = a.associator(b, c);

// Mathematical conjugation
let a_conj = a.conj();

// Bitwise operations applied across the 8D vector natively
let masked = a ^ b.rotate_left(3);
```

## License

Distributed under the **GNU AGPL v3.0**. See the `LICENSE` file for full details.

<br>

---

<div align="center">
  <img src="static/cycleless_100.png" alt="Cycleless Logo" width="60" />
  <p><b>CYCLELESS RnD</b><br><i>Zero Cycles. Absolute Precision.</i></p>
</div>

