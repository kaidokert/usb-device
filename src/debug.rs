use core::sync::atomic::{AtomicU32, Ordering};

#[derive(Copy, Clone, Debug, Default)]
pub struct DebugSnapshot {
    pub set_address_match_count: u32,
    pub set_address_accept_attempt_count: u32,
    pub set_address_accept_ok_count: u32,
    pub set_address_accept_err_count: u32,
    pub get_descriptor_enter_count: u32,
    pub get_descriptor_ok_count: u32,
    pub get_descriptor_err_count: u32,
    pub descriptor_accept_enter_count: u32,
    pub descriptor_accept_ok_count: u32,
    pub descriptor_accept_err_count: u32,
    pub accept_in_enter_count: u32,
    pub accept_in_ok_count: u32,
    pub accept_in_err_count: u32,
    pub accept_out_enter_count: u32,
    pub accept_out_invalid_state_count: u32,
    pub accept_out_write_ok_count: u32,
    pub in_complete_enter_count: u32,
    pub in_complete_data_in_count: u32,
    pub in_complete_data_in_zlp_count: u32,
    pub in_complete_data_in_last_count: u32,
    pub in_complete_status_in_count: u32,
    pub in_complete_idle_count: u32,
    pub in_complete_other_count: u32,
    pub handle_out_enter_count: u32,
    pub handle_out_data_out_count: u32,
    pub handle_out_status_out_count: u32,
    pub handle_out_other_count: u32,
    pub last_in_complete_state: u32,
    pub last_handle_out_state: u32,
}

static SET_ADDRESS_MATCH_COUNT: AtomicU32 = AtomicU32::new(0);
static SET_ADDRESS_ACCEPT_ATTEMPT_COUNT: AtomicU32 = AtomicU32::new(0);
static SET_ADDRESS_ACCEPT_OK_COUNT: AtomicU32 = AtomicU32::new(0);
static SET_ADDRESS_ACCEPT_ERR_COUNT: AtomicU32 = AtomicU32::new(0);
static GET_DESCRIPTOR_ENTER_COUNT: AtomicU32 = AtomicU32::new(0);
static GET_DESCRIPTOR_OK_COUNT: AtomicU32 = AtomicU32::new(0);
static GET_DESCRIPTOR_ERR_COUNT: AtomicU32 = AtomicU32::new(0);
static DESCRIPTOR_ACCEPT_ENTER_COUNT: AtomicU32 = AtomicU32::new(0);
static DESCRIPTOR_ACCEPT_OK_COUNT: AtomicU32 = AtomicU32::new(0);
static DESCRIPTOR_ACCEPT_ERR_COUNT: AtomicU32 = AtomicU32::new(0);
static ACCEPT_IN_ENTER_COUNT: AtomicU32 = AtomicU32::new(0);
static ACCEPT_IN_OK_COUNT: AtomicU32 = AtomicU32::new(0);
static ACCEPT_IN_ERR_COUNT: AtomicU32 = AtomicU32::new(0);
static ACCEPT_OUT_ENTER_COUNT: AtomicU32 = AtomicU32::new(0);
static ACCEPT_OUT_INVALID_STATE_COUNT: AtomicU32 = AtomicU32::new(0);
static ACCEPT_OUT_WRITE_OK_COUNT: AtomicU32 = AtomicU32::new(0);
static IN_COMPLETE_ENTER_COUNT: AtomicU32 = AtomicU32::new(0);
static IN_COMPLETE_DATA_IN_COUNT: AtomicU32 = AtomicU32::new(0);
static IN_COMPLETE_DATA_IN_ZLP_COUNT: AtomicU32 = AtomicU32::new(0);
static IN_COMPLETE_DATA_IN_LAST_COUNT: AtomicU32 = AtomicU32::new(0);
static IN_COMPLETE_STATUS_IN_COUNT: AtomicU32 = AtomicU32::new(0);
static IN_COMPLETE_IDLE_COUNT: AtomicU32 = AtomicU32::new(0);
static IN_COMPLETE_OTHER_COUNT: AtomicU32 = AtomicU32::new(0);
static HANDLE_OUT_ENTER_COUNT: AtomicU32 = AtomicU32::new(0);
static HANDLE_OUT_DATA_OUT_COUNT: AtomicU32 = AtomicU32::new(0);
static HANDLE_OUT_STATUS_OUT_COUNT: AtomicU32 = AtomicU32::new(0);
static HANDLE_OUT_OTHER_COUNT: AtomicU32 = AtomicU32::new(0);
static LAST_IN_COMPLETE_STATE: AtomicU32 = AtomicU32::new(0);
static LAST_HANDLE_OUT_STATE: AtomicU32 = AtomicU32::new(0);

pub fn reset() {
    SET_ADDRESS_MATCH_COUNT.store(0, Ordering::Release);
    SET_ADDRESS_ACCEPT_ATTEMPT_COUNT.store(0, Ordering::Release);
    SET_ADDRESS_ACCEPT_OK_COUNT.store(0, Ordering::Release);
    SET_ADDRESS_ACCEPT_ERR_COUNT.store(0, Ordering::Release);
    GET_DESCRIPTOR_ENTER_COUNT.store(0, Ordering::Release);
    GET_DESCRIPTOR_OK_COUNT.store(0, Ordering::Release);
    GET_DESCRIPTOR_ERR_COUNT.store(0, Ordering::Release);
    DESCRIPTOR_ACCEPT_ENTER_COUNT.store(0, Ordering::Release);
    DESCRIPTOR_ACCEPT_OK_COUNT.store(0, Ordering::Release);
    DESCRIPTOR_ACCEPT_ERR_COUNT.store(0, Ordering::Release);
    ACCEPT_IN_ENTER_COUNT.store(0, Ordering::Release);
    ACCEPT_IN_OK_COUNT.store(0, Ordering::Release);
    ACCEPT_IN_ERR_COUNT.store(0, Ordering::Release);
    ACCEPT_OUT_ENTER_COUNT.store(0, Ordering::Release);
    ACCEPT_OUT_INVALID_STATE_COUNT.store(0, Ordering::Release);
    ACCEPT_OUT_WRITE_OK_COUNT.store(0, Ordering::Release);
    IN_COMPLETE_ENTER_COUNT.store(0, Ordering::Release);
    IN_COMPLETE_DATA_IN_COUNT.store(0, Ordering::Release);
    IN_COMPLETE_DATA_IN_ZLP_COUNT.store(0, Ordering::Release);
    IN_COMPLETE_DATA_IN_LAST_COUNT.store(0, Ordering::Release);
    IN_COMPLETE_STATUS_IN_COUNT.store(0, Ordering::Release);
    IN_COMPLETE_IDLE_COUNT.store(0, Ordering::Release);
    IN_COMPLETE_OTHER_COUNT.store(0, Ordering::Release);
    HANDLE_OUT_ENTER_COUNT.store(0, Ordering::Release);
    HANDLE_OUT_DATA_OUT_COUNT.store(0, Ordering::Release);
    HANDLE_OUT_STATUS_OUT_COUNT.store(0, Ordering::Release);
    HANDLE_OUT_OTHER_COUNT.store(0, Ordering::Release);
    LAST_IN_COMPLETE_STATE.store(0, Ordering::Release);
    LAST_HANDLE_OUT_STATE.store(0, Ordering::Release);
}

pub fn snapshot() -> DebugSnapshot {
    DebugSnapshot {
        set_address_match_count: SET_ADDRESS_MATCH_COUNT.load(Ordering::Acquire),
        set_address_accept_attempt_count: SET_ADDRESS_ACCEPT_ATTEMPT_COUNT.load(Ordering::Acquire),
        set_address_accept_ok_count: SET_ADDRESS_ACCEPT_OK_COUNT.load(Ordering::Acquire),
        set_address_accept_err_count: SET_ADDRESS_ACCEPT_ERR_COUNT.load(Ordering::Acquire),
        get_descriptor_enter_count: GET_DESCRIPTOR_ENTER_COUNT.load(Ordering::Acquire),
        get_descriptor_ok_count: GET_DESCRIPTOR_OK_COUNT.load(Ordering::Acquire),
        get_descriptor_err_count: GET_DESCRIPTOR_ERR_COUNT.load(Ordering::Acquire),
        descriptor_accept_enter_count: DESCRIPTOR_ACCEPT_ENTER_COUNT.load(Ordering::Acquire),
        descriptor_accept_ok_count: DESCRIPTOR_ACCEPT_OK_COUNT.load(Ordering::Acquire),
        descriptor_accept_err_count: DESCRIPTOR_ACCEPT_ERR_COUNT.load(Ordering::Acquire),
        accept_in_enter_count: ACCEPT_IN_ENTER_COUNT.load(Ordering::Acquire),
        accept_in_ok_count: ACCEPT_IN_OK_COUNT.load(Ordering::Acquire),
        accept_in_err_count: ACCEPT_IN_ERR_COUNT.load(Ordering::Acquire),
        accept_out_enter_count: ACCEPT_OUT_ENTER_COUNT.load(Ordering::Acquire),
        accept_out_invalid_state_count: ACCEPT_OUT_INVALID_STATE_COUNT.load(Ordering::Acquire),
        accept_out_write_ok_count: ACCEPT_OUT_WRITE_OK_COUNT.load(Ordering::Acquire),
        in_complete_enter_count: IN_COMPLETE_ENTER_COUNT.load(Ordering::Acquire),
        in_complete_data_in_count: IN_COMPLETE_DATA_IN_COUNT.load(Ordering::Acquire),
        in_complete_data_in_zlp_count: IN_COMPLETE_DATA_IN_ZLP_COUNT.load(Ordering::Acquire),
        in_complete_data_in_last_count: IN_COMPLETE_DATA_IN_LAST_COUNT.load(Ordering::Acquire),
        in_complete_status_in_count: IN_COMPLETE_STATUS_IN_COUNT.load(Ordering::Acquire),
        in_complete_idle_count: IN_COMPLETE_IDLE_COUNT.load(Ordering::Acquire),
        in_complete_other_count: IN_COMPLETE_OTHER_COUNT.load(Ordering::Acquire),
        handle_out_enter_count: HANDLE_OUT_ENTER_COUNT.load(Ordering::Acquire),
        handle_out_data_out_count: HANDLE_OUT_DATA_OUT_COUNT.load(Ordering::Acquire),
        handle_out_status_out_count: HANDLE_OUT_STATUS_OUT_COUNT.load(Ordering::Acquire),
        handle_out_other_count: HANDLE_OUT_OTHER_COUNT.load(Ordering::Acquire),
        last_in_complete_state: LAST_IN_COMPLETE_STATE.load(Ordering::Acquire),
        last_handle_out_state: LAST_HANDLE_OUT_STATE.load(Ordering::Acquire),
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

pub(crate) fn note_get_descriptor_enter() {
    GET_DESCRIPTOR_ENTER_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_get_descriptor_ok() {
    GET_DESCRIPTOR_OK_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_get_descriptor_err() {
    GET_DESCRIPTOR_ERR_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_descriptor_accept_enter() {
    DESCRIPTOR_ACCEPT_ENTER_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_descriptor_accept_ok() {
    DESCRIPTOR_ACCEPT_OK_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_descriptor_accept_err() {
    DESCRIPTOR_ACCEPT_ERR_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_accept_in_enter() {
    ACCEPT_IN_ENTER_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_accept_in_ok() {
    ACCEPT_IN_OK_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_accept_in_err() {
    ACCEPT_IN_ERR_COUNT.fetch_add(1, Ordering::AcqRel);
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

pub(crate) fn note_in_complete_enter() {
    IN_COMPLETE_ENTER_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_in_complete_state(state: u32) {
    LAST_IN_COMPLETE_STATE.store(state, Ordering::Release);
}

pub(crate) fn note_in_complete_data_in() {
    IN_COMPLETE_DATA_IN_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_in_complete_data_in_zlp() {
    IN_COMPLETE_DATA_IN_ZLP_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_in_complete_data_in_last() {
    IN_COMPLETE_DATA_IN_LAST_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_in_complete_status_in() {
    IN_COMPLETE_STATUS_IN_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_in_complete_idle() {
    IN_COMPLETE_IDLE_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_in_complete_other() {
    IN_COMPLETE_OTHER_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_handle_out_enter() {
    HANDLE_OUT_ENTER_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_handle_out_state(state: u32) {
    LAST_HANDLE_OUT_STATE.store(state, Ordering::Release);
}

pub(crate) fn note_handle_out_data_out() {
    HANDLE_OUT_DATA_OUT_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_handle_out_status_out() {
    HANDLE_OUT_STATUS_OUT_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn note_handle_out_other() {
    HANDLE_OUT_OTHER_COUNT.fetch_add(1, Ordering::AcqRel);
}
