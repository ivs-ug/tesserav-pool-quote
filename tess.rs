#[derive(Debug, Clone, Copy)]
pub struct Tick {
    pub capacity: u64,   // offset 0x00
    pub multiplier: u64, // offset 0x08
    pub active: u8,      // offset 0x10
}

#[derive(Debug, Clone)]
pub struct Pool {
    pub pool_pubkey: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub vault_id_a: u16,
    pub vault_id_b: u16,
    pub price_a: u128,
    pub price_b: u128,
    pub ticks_atb: [Tick; TICK_COUNT],
    pub ticks_bta: [Tick; TICK_COUNT],
    pub dyn_fee_flag: bool,
}

impl Pool {
    pub fn from_bytes(data: &[u8], pool_pubkey: Pubkey) -> Result<Self> {
        if data.len() < ACCOUNT_MIN_LEN {
            return Err(eyre!("too short: {} < {ACCOUNT_MIN_LEN}", data.len()));
        }

        let vault_id_a = rd_u16(data, OFF_VAULT_ID_A);
        let vault_id_b = rd_u16(data, OFF_VAULT_ID_B);

        let mut ticks_atb = [Tick {
            capacity: 0,
            multiplier: 0,
            active: 0,
        }; TICK_COUNT];
        for i in 0..TICK_COUNT {
            let b = OFF_TICKS_ATB + i * TICK_STRIDE;
            ticks_atb[i] = Tick {
                capacity: rd_u64(data, b),
                multiplier: rd_u64(data, b + 8),
                active: rd_u8(data, b + 16),
            };
        }

        let mut ticks_bta = [Tick {
            capacity: 0,
            multiplier: 0,
            active: 0,
        }; TICK_COUNT];
        for i in 0..TICK_COUNT {
            let b = OFF_TICKS_BTA + i * TICK_STRIDE;
            ticks_bta[i] = Tick {
                capacity: rd_u64(data, b),
                multiplier: rd_u64(data, b + 8),
                active: rd_u8(data, b + 16),
            };
        }

        Ok(Pool {
            pool_pubkey,
            mint_a: rd_pubkey(data, OFF_MINT_A),
            mint_b: rd_pubkey(data, OFF_MINT_B),
            vault_a: derive_vault_pubkey(vault_id_a),
            vault_b: derive_vault_pubkey(vault_id_b),
            reserve_a: rd_u64(data, OFF_RESERVE_A),
            reserve_b: rd_u64(data, OFF_RESERVE_B),
            vault_id_a,
            vault_id_b,
            price_a: rd_u128(data, OFF_PRICE_A),
            price_b: rd_u128(data, OFF_PRICE_B),
            ticks_atb,
            ticks_bta,
            dyn_fee_flag: rd_u8(data, OFF_DYN_FEE_FLAG) == 1,
        })
    }
}

pub fn quote(
    amount_in: u64,
    _reserve_a: u64,
    _reserve_b: u64,
    a_to_b: bool,
    pool: &Pool,
    _current_slot: u64,
    out_vault_balance: u64, // Cap for output amount
) -> Option<u64> {
  // ...
}
