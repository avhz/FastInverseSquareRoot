# FastInverseSquareRoot
Quake III Fast Inverse Square Root in Rust.

# The original C version by id Software

```c
float Q_rsqrt( float number )
{
	long i;
	float x2, y;
	const float threehalfs = 1.5F;

	x2 = number * 0.5F;
	y  = number;
	i  = * ( long * ) &y;                       // evil floating point bit level hacking
	i  = 0x5f3759df - ( i >> 1 );               // what the fuck? 
	y  = * ( float * ) &i;
	y  = y * ( threehalfs - ( x2 * y * y ) );   // 1st iteration
//	y  = y * ( threehalfs - ( x2 * y * y ) );   // 2nd iteration, this can be removed

	return y;
}
```

# Rust implementation

```rust
fn inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    y * (1.5 - 0.5 * x * y * y)
}
```
# Assembly comparison

# C ASM

```assembly
.LCPI0_0:
        .long   3204448256              # float -0.5
.LCPI0_1:
        .long   1069547520              # float 1.5
InvSqrt:                                # @InvSqrt
        movd    eax, xmm0
        mulss   xmm0, dword ptr [rip + .LCPI0_0]
        sar     eax
        mov     ecx, 1597463007
        sub     ecx, eax
        movd    xmm1, ecx
        mulss   xmm0, xmm1
        mulss   xmm0, xmm1
        addss   xmm0, dword ptr [rip + .LCPI0_1]
        mulss   xmm0, xmm1
        ret
```

# Rust ASM

```assembly
.LCPI0_0:
        .long   3204448256        ; f32 -0.5
.LCPI0_1:
        .long   1069547520        ; f32  1.5
example::inv_sqrt:
        movd    eax, xmm0
        shr     eax                   ; i << 1
        mov     ecx, 1597463007       ; 0x5f3759df
        sub     ecx, eax              ; 0x5f3759df - ...
        movd    xmm1, ecx
        mulss   xmm0, dword ptr [rip + .LCPI0_0]    ; x *= 0.5
        mulss   xmm0, xmm1                          ; x *= y
        mulss   xmm0, xmm1                          ; x *= y
        addss   xmm0, dword ptr [rip + .LCPI0_1]    ; x += 1.5
        mulss   xmm0, xmm1                          ; x *= y
        ret
```
