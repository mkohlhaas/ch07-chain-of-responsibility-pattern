// Minimal Idiomatic Example
//
// Complete, idiomatic Rust implementation representing a support ticket routing system (First Line Support → Supervisor)

// =================================== //
// 1. Define the Request using an enum //
// =================================== //

#[derive(Debug, Clone)]
pub enum SupportRequest {
    PasswordReset,
    BillingIssue,
    CorporateContract,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HandleResult {
    Handled,
    Escalated,
    Unhandled,
}

// ============================================================ //
// 2. Define the Handler trait with dynamic dispatch capability //
// ============================================================ //

pub trait Handler {
    fn handle(&self, request: &SupportRequest) -> HandleResult;
    fn set_next_handler(&mut self, next: Box<dyn Handler>);
}

// Shared chain-walking logic used by all concrete handlers.
fn walk_chain(next_handler: Option<&Box<dyn Handler>>, request: &SupportRequest) -> HandleResult {
    match next_handler {
        Some(next) => match next.handle(request) {
            HandleResult::Handled | HandleResult::Escalated => HandleResult::Escalated,
            HandleResult::Unhandled => HandleResult::Unhandled,
        },
        None => HandleResult::Unhandled,
    }
}

// =========================== //
// 3. Create Concrete Handlers //
// =========================== //

// =================== //
// A. FirstLineSupport //
// =================== //

#[derive(Default)]
pub struct FirstLineSupport {
    next_handler: Option<Box<dyn Handler>>,
}

impl Handler for FirstLineSupport {
    fn set_next_handler(&mut self, next: Box<dyn Handler>) {
        self.next_handler = Some(next);
    }

    // FirstLineSupport can only handle PasswordResets
    fn handle(&self, request: &SupportRequest) -> HandleResult {
        if matches!(request, SupportRequest::PasswordReset) {
            println!("FirstLineSupport: Resolved the PasswordReset request.");
            HandleResult::Handled
        } else if self.next_handler.is_some() {
            println!("FirstLineSupport: Cannot handle. Passing to next...");
            walk_chain(self.next_handler.as_ref(), request)
        } else {
            println!("FirstLineSupport: Reached end of chain. Request unhandled.");
            HandleResult::Unhandled
        }
    }
}

// ============= //
// B. Supervisor //
// ============= //

#[derive(Default)]
pub struct Supervisor {
    next_handler: Option<Box<dyn Handler>>,
}

impl Handler for Supervisor {
    fn set_next_handler(&mut self, next: Box<dyn Handler>) {
        self.next_handler = Some(next);
    }

    // Supervisor can only handle BillingIssues
    fn handle(&self, request: &SupportRequest) -> HandleResult {
        if matches!(request, SupportRequest::BillingIssue) {
            println!("Supervisor: Resolved the BillingIssue request.");
            HandleResult::Handled
        } else if self.next_handler.is_some() {
            println!("Supervisor: Cannot handle. Passing to next...");
            walk_chain(self.next_handler.as_ref(), request)
        } else {
            println!("Supervisor: Cannot handle. Reached end of chain. Request unhandled.");
            HandleResult::Unhandled
        }
    }
}

// ================================== //
// 4. Construct the chain and execute //
// ================================== //

fn main() {
    let mut first = FirstLineSupport::default();
    let supervisor = Supervisor::default();

    // Link the chain together
    first.set_next_handler(Box::new(supervisor));

    // Test a request handled at the start
    println!("== Submitting Password Reset ==");
    println!("{:?}", first.handle(&SupportRequest::PasswordReset));

    // Test a request escalated down the chain
    println!("\n== Submitting Billing Issue ==");
    println!("{:?}", first.handle(&SupportRequest::BillingIssue));

    // Test an unhandled request
    println!("\n== Submitting Corporate Contract ==");
    println!("{:?}", first.handle(&SupportRequest::CorporateContract));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_chain() -> FirstLineSupport {
        let mut first = FirstLineSupport::default();
        let supervisor = Supervisor::default();
        first.set_next_handler(Box::new(supervisor));
        first
    }

    #[test]
    fn password_reset_handled_at_first_line() {
        assert_eq!(
            build_chain().handle(&SupportRequest::PasswordReset),
            HandleResult::Handled
        );
    }

    #[test]
    fn billing_issue_escalated_to_supervisor() {
        assert_eq!(
            build_chain().handle(&SupportRequest::BillingIssue),
            HandleResult::Escalated
        );
    }

    #[test]
    fn corporate_contract_unhandled_at_end_of_chain() {
        assert_eq!(
            build_chain().handle(&SupportRequest::CorporateContract),
            HandleResult::Unhandled
        );
    }

    #[test]
    fn supervisor_handles_billing_issue_directly() {
        let supervisor = Supervisor::default();
        assert_eq!(
            supervisor.handle(&SupportRequest::BillingIssue),
            HandleResult::Handled
        );
    }

    #[test]
    fn supervisor_cannot_handle_password_reset_alone() {
        let supervisor = Supervisor::default();
        assert_eq!(
            supervisor.handle(&SupportRequest::PasswordReset),
            HandleResult::Unhandled
        );
    }

    #[test]
    fn lone_first_line_cannot_handle_billing_issue() {
        let first = FirstLineSupport::default();
        assert_eq!(
            first.handle(&SupportRequest::BillingIssue),
            HandleResult::Unhandled
        );
    }

    #[test]
    fn supervisor_escalates_through_a_tail_handler() {
        let mut supervisor = Supervisor::default();
        let tail = FirstLineSupport::default();
        supervisor.set_next_handler(Box::new(tail));

        // Supervisor can't handle PasswordReset, so it escalates to tail...
        assert_eq!(
            supervisor.handle(&SupportRequest::PasswordReset),
            HandleResult::Escalated
        );
    }
}
