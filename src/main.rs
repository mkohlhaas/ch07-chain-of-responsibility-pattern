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

/// Outcome of routing a request through the chain.
#[derive(Debug, PartialEq, Eq)]
pub enum HandleResult {
    /// Resolved by the handler that was invoked.
    Handled,
    /// Not resolved by the invoked handler, but resolved downstream.
    Escalated,
    /// No handler in the chain could resolve the request.
    Unhandled,
}

impl HandleResult {
    /// Whether the request was resolved somewhere in the chain.
    fn is_resolved(&self) -> bool {
        matches!(self, HandleResult::Handled | HandleResult::Escalated)
    }
}

// ============================================================ //
// 2. Define the Handler trait with dynamic dispatch capability //
// ============================================================ //

pub trait Handler {
    fn handle(&self, request: &SupportRequest) -> HandleResult;
    fn set_next_handler(&mut self, next: Box<dyn Handler>);
}

// A handler that cannot resolve a request forwards it to the next handler in
// the chain. Resolving anywhere downstream counts as an escalation from the
// forwarding handler's point of view.
fn walk_chain(next_handler: Option<&Box<dyn Handler>>, request: &SupportRequest) -> HandleResult {
    match next_handler {
        Some(next) => {
            if next.handle(request).is_resolved() {
                HandleResult::Escalated
            } else {
                HandleResult::Unhandled
            }
        }
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
            HandleResult::Handled
        } else {
            walk_chain(self.next_handler.as_ref(), request)
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
            HandleResult::Handled
        } else {
            walk_chain(self.next_handler.as_ref(), request)
        }
    }
}

// ================================== //
// 4. Construct the chain and execute //
// ================================== //

fn main() {
    let mut first = FirstLineSupport::default();
    first.set_next_handler(Box::new(Supervisor::default()));

    // Handled at the start of the chain
    println!("== Submitting Password Reset ==");
    println!("{:?}", first.handle(&SupportRequest::PasswordReset));

    // Handled after escalation to the supervisor
    println!("\n== Submitting Billing Issue ==");
    println!("{:?}", first.handle(&SupportRequest::BillingIssue));

    // Falls off the end of the chain, unhandled
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
