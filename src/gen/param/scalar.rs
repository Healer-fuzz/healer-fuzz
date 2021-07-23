use crate::gen::*;

const MAGIC32: [u64; 24] = [
    0,             //
    1,             //
    16,            // One-off with common buffer size
    32,            // One-off with common buffer size
    64,            // One-off with common buffer size
    100,           // One-off with common buffer size
    127,           // Overflow signed 8-bit when incremented
    128,           // Overflow signed 8-bit when decremented
    255,           // -1
    256,           // Overflow unsig 8-bit
    512,           // One-off with common buffer size
    1000,          // One-off with common buffer size
    1024,          // One-off with common buffer size
    4096,          // One-off with common buffer size
    32767,         // Overflow signed 16-bit when incremented
    32768,         // Overflow signed 16-bit when decremented
    65407,         // Overflow signed 8-bit
    65535,         // Overflow unsig 16-bit when incremented
    65536,         // Overflow unsig 16 bit
    100_663_045,   // Large positive number (endian-agnostic)
    2_147_483_647, // Overflow signed 32-bit when incremented
    2_147_483_648, // Overflow signed 32-bit when decremented
    4_194_304_250, // Large negative number (endian-agnostic)
    4_294_934_527, // Overflow signed 16-bit
];

pub const MAGIC64: [u64; 24 * 24] = {
    let mut magic = [0; 24 * 24];
    let mut i = 0;
    let mut j = 0;
    while i != 24 {
        while j != 24 {
            magic[i * 24 + j] = (MAGIC32[i] << 32) | MAGIC32[j];
            j += 1;
        }
        i += 1;
    }
    magic
};

pub(super) fn gen(ctx: &mut ProgContext, ty: TypeRef, dir: Dir) -> Value {
    use TypeKind::*;
    let val = match &ty.kind {
        Int { range, align, .. } => gen_integer(ty.bit_len().unwrap(), *range, *align),
        Flags { vals, bitmask, .. } => gen_flag(vals, *bitmask),
        Proc { per_proc, .. } => thread_rng().gen_range(0..*per_proc),
        Csum { .. } => gen_integer(ty.bit_len().unwrap(), None, 0),
        Len { .. } => {
            ctx.record_len_to_param_ctx(); // Mark here, calculate later.
            0
        }
        Const { val, .. } => *val,
        _ => unreachable!(),
    };
    Value::new(dir, ty, ValueKind::new_scalar(val))
}

/// Generate a random integer value of 'bit' size, in range 'range', with alignment 'align'.
pub fn gen_integer(mut bit: u64, range: Option<(u64, u64)>, mut align: u64) -> u64 {
    if align == 0 {
        align = 1;
    }
    if bit == 0 || bit > 64 {
        bit = 64;
    }
    let (min, max) = range.map(filter_range).unwrap_or((u64::MIN, u64::MAX));
    let mask = if bit == 64 { u64::MAX } else { (1 << bit) - 1 };

    let mut rng = thread_rng();
    let val: u64 = rng.gen_range(min..max);
    let shift_align = |mut val: u64| {
        val -= val % align;
        val &= mask;
        val
    };

    if rng.gen_bool(0.65) {
        shift_align(val)
    } else {
        let magic = MAGIC64
            .iter()
            .copied()
            .filter(|&n| n >= min && n < max)
            .choose(&mut rng)
            .unwrap_or(val);

        shift_align(magic + (val & 1))
    }
}

pub fn filter_range((min, max): (u64, u64)) -> (u64, u64) {
    if min >= max {
        (u64::MIN, u64::MAX)
    } else {
        (min, max)
    }
}

/// Generate a random flag value.
pub fn gen_flag(vals: &[u64], bitmask: bool) -> u64 {
    let mut rng = thread_rng();
    let extra_vals = || MAGIC64.iter().copied();

    let mut val = 0;
    if !bitmask {
        if rng.gen_ratio(5, 1000) {
            if rng.gen::<bool>() {
                val
            } else {
                rng.gen::<u64>()
            }
        } else if rng.gen_ratio(1, 100) {
            extra_vals().choose(&mut rng).unwrap()
        } else {
            let vals = extra_vals().chain(vals.iter().copied());
            let mut val = vals.choose(&mut rng).unwrap();
            if rng.gen_ratio(1, 10) {
                val ^= MAGIC64.iter().copied().choose(&mut rng).unwrap();
            }
            val
        }
    } else {
        let mut t = 0.8;
        while val == 0 || rng.gen_bool(t) {
            let mut flag = if !vals.is_empty() && rng.gen_ratio(8, 10) {
                vals.iter().copied().choose(&mut rng).unwrap()
            } else {
                extra_vals().choose(&mut rng).unwrap()
            };
            if rng.gen_ratio(5, 100) {
                if rng.gen::<bool>() {
                    flag >>= 1;
                } else {
                    flag <<= 1;
                }
            }
            val ^= flag;
            t /= 2.0;
        }
        val
    }
}
