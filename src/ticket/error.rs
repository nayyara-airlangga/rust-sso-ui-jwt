//! Ticket validation errors.

/// Types of errors that could happen in ticket validation.
#[derive(Debug)]
pub enum ValidateTicketError {
    /// Failed ticket authentication.
    AuthenticationFailed,
    /// Errors regarding the validation request.
    ReqwestError,
    /// A bad request to the CAS server.
    BadRequest,
    /// Error parsing the XML response.
    XMLParsingError,
}
