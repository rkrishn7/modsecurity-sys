/* automatically generated by rust-bindgen 0.69.4 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModSecurity_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModSecurityIntervention_t {
    pub status: ::std::os::raw::c_int,
    pub pause: ::std::os::raw::c_int,
    pub url: *mut ::std::os::raw::c_char,
    pub log: *mut ::std::os::raw::c_char,
    pub disruptive: ::std::os::raw::c_int,
}
pub type ModSecurityIntervention = ModSecurityIntervention_t;
pub type ModSecurity = ModSecurity_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transaction_t {
    _unused: [u8; 0],
}
pub type Transaction = Transaction_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rules_t {
    _unused: [u8; 0],
}
pub type RulesSet = Rules_t;
extern "C" {
    pub fn msc_new_transaction(
        ms: *mut ModSecurity,
        rules: *mut RulesSet,
        logCbData: *mut ::std::os::raw::c_void,
    ) -> *mut Transaction;
}
extern "C" {
    pub fn msc_new_transaction_with_id(
        ms: *mut ModSecurity,
        rules: *mut RulesSet,
        id: *mut ::std::os::raw::c_char,
        logCbData: *mut ::std::os::raw::c_void,
    ) -> *mut Transaction;
}
extern "C" {
    pub fn msc_process_connection(
        transaction: *mut Transaction,
        client: *const ::std::os::raw::c_char,
        cPort: ::std::os::raw::c_int,
        server: *const ::std::os::raw::c_char,
        sPort: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_process_request_headers(transaction: *mut Transaction) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_add_request_header(
        transaction: *mut Transaction,
        key: *const ::std::os::raw::c_uchar,
        value: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_add_n_request_header(
        transaction: *mut Transaction,
        key: *const ::std::os::raw::c_uchar,
        len_key: usize,
        value: *const ::std::os::raw::c_uchar,
        len_value: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_process_request_body(transaction: *mut Transaction) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_append_request_body(
        transaction: *mut Transaction,
        body: *const ::std::os::raw::c_uchar,
        size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_request_body_from_file(
        transaction: *mut Transaction,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_process_response_headers(
        transaction: *mut Transaction,
        code: ::std::os::raw::c_int,
        protocol: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_add_response_header(
        transaction: *mut Transaction,
        key: *const ::std::os::raw::c_uchar,
        value: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_add_n_response_header(
        transaction: *mut Transaction,
        key: *const ::std::os::raw::c_uchar,
        len_key: usize,
        value: *const ::std::os::raw::c_uchar,
        len_value: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_process_response_body(transaction: *mut Transaction) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_append_response_body(
        transaction: *mut Transaction,
        body: *const ::std::os::raw::c_uchar,
        size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_process_uri(
        transaction: *mut Transaction,
        uri: *const ::std::os::raw::c_char,
        protocol: *const ::std::os::raw::c_char,
        http_version: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_get_response_body(transaction: *mut Transaction) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn msc_get_response_body_length(transaction: *mut Transaction) -> usize;
}
extern "C" {
    pub fn msc_get_request_body_length(transaction: *mut Transaction) -> usize;
}
extern "C" {
    pub fn msc_transaction_cleanup(transaction: *mut Transaction);
}
extern "C" {
    pub fn msc_intervention(
        transaction: *mut Transaction,
        it: *mut ModSecurityIntervention,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_process_logging(transaction: *mut Transaction) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_update_status_code(
        transaction: *mut Transaction,
        status: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type ModSecLogCb = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *const ::std::os::raw::c_void),
>;
extern "C" {
    pub fn msc_init() -> *mut ModSecurity;
}
extern "C" {
    pub fn msc_who_am_i(msc: *mut ModSecurity) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn msc_set_connector_info(msc: *mut ModSecurity, connector: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn msc_set_log_cb(msc: *mut ModSecurity, cb: ModSecLogCb);
}
extern "C" {
    pub fn msc_cleanup(msc: *mut ModSecurity);
}
extern "C" {
    pub fn msc_create_rules_set() -> *mut RulesSet;
}
extern "C" {
    pub fn msc_rules_dump(rules: *mut RulesSet);
}
extern "C" {
    pub fn msc_rules_merge(
        rules_dst: *mut RulesSet,
        rules_from: *mut RulesSet,
        error: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_rules_add_remote(
        rules: *mut RulesSet,
        key: *const ::std::os::raw::c_char,
        uri: *const ::std::os::raw::c_char,
        error: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_rules_add_file(
        rules: *mut RulesSet,
        file: *const ::std::os::raw::c_char,
        error: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_rules_add(
        rules: *mut RulesSet,
        plain_rules: *const ::std::os::raw::c_char,
        error: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn msc_rules_cleanup(rules: *mut RulesSet) -> ::std::os::raw::c_int;
}