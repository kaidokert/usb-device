use core::sync::atomic::{AtomicU32, Ordering};

#[derive(Copy, Clone, Debug, Default)]
pub struct DebugSnapshot {
    pub set_address_match_count: u32,
    pub set_address_accept_attempt_count: u32,
    pub set_address_accept_ok_count: u32,
    pub set_address_accept_err_count: u32,
    pub accept_out_enter_count: u32,
    pub accept_out_invalid_state_count: u32,
    pub accept_out_write_ok_count: u32,
}

static SET_ADDRESS_MATCH_COUNT: AtomicU32 = AtomicU32::new(0);
static SET_ADDRESS_ACCEPT_ATTEMPT_COUNT: AtomicU32 = AtomicU32::new(0);
static SET_ADDRESS_ACCEPT_OK_COUNT: AtomicU32 = AtomicU32::new(0);
static SET_ADDRESS_ACCEPT_ERR_COUNT: AtomicU32 = AtomicU32::new(0);
static ACCEPT_OUT_ENTER_COUNT: AtomicU32 = AtomicU32::new(0);
static ACCEPT_OUT_INVALID_STATE_COUNT: AtomicU32 = AtomicU32::new(0);
static ACCEPT_OUT_WRITE_OK_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn reset() {
    SET_ADDRESS_MATCH_COUNT.store(0, Ordering::Release);
    SET_ADDRESS_ACCEPT_ATTEMPT_COUNT.store(0, Ordering::Release);
    SET_ADDRESS_ACCEPT_OK_COUNT.store(0, Ordering::Release);
    SET_ADDRESS_ACCEPT_ERR_COUNT.store(0, Ordering::Release);
    ACCEPT_OUT_ENTER_COUNT.store(0, Ordering::Release);
    ACCEPT_OUT_INVALID_STATE_COUNT.store(0, Ordering::Release);
    ACCEPT_OUT_WRITE_OK_COUNT.store(0, Ordering::Release);
}

pub fn snapshot() -> DebugSnapshot {
    DebugSnapshot {
        set_address_match_count: SET_ADDRESS_MATCH_COUNT.load(Ordering::Acquire),
        set_address_accept_attempt_count: SET_ADDRESS_ACCEPT_ATTEMPT_COUNT.load(Ordering::Acquire),
        set_address_accept_ok_count: SET_ADDRESS_ACCEPT_OK_COUNT.load(Ordering::Acquire),
        set_address_accept_err_count: SET_ADDRESS_ACCEPT_ERR_COUNT.load(Ordering::Acquire),
        accept_out_enter_count: ACCEPT_OUT_ENTER_COUNT.load(Ordering::Acquire),
        accept_out_invalid_state_count: ACCEPT_OUT_INVALID_STATE_COUNT.load(Ordering::Acquire),
        accept_out_write_ok_count: ACCEPT_OUT_WRITE_OK_COUNT.load(Ordering::Acquire),
    }
}

pub(crate) fn note_set_address_match() {
    SET_ADDRESS_MATCH_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_set_address_accept_attempt() {
    SET_ADDRESS_ACCEPT_ATTEMPT_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_set_address_accept_ok() {
    SET_ADDRESS_ACCEPT_OK_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_set_address_accept_err() {
    SET_ADDRESS_ACCEPT_ERR_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_accept_out_enter() {
    ACCEPT_OUT_ENTER_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_accept_out_invalid_state() {
    ACCEPT_OUT_INVALID_STATE_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_accept_out_write_ok() {
    ACCEPT_OUT_WRITE_OK_COUNT.fetch_add(1, Ordering::AcqRel);
}
