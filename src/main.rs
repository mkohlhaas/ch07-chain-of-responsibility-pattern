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

// ============================================================ //
// 2. Define the Handler trait with dynamic dispatch capability //
// ============================================================ //

pub trait Handler {
    fn handle(&self, request: &SupportRequest);
    fn set_next_handler(&mut self, next: Box<dyn Handler>);
}

// =========================== //
// 3. Create Concrete Handlers //
// =========================== //

// =================== //
// A. FirstLineSupport //
// =================== //

pub struct FirstLineSupport {
    next_handler: Option<Box<dyn Handler>>,
}

impl Default for FirstLineSupport {
    fn default() -> Self {
        Self::new()
    }
}

impl FirstLineSupport {
    pub fn new() -> Self {
        Self { next_handler: None }
    }
}

impl Handler for FirstLineSupport {
    fn set_next_handler(&mut self, next: Box<dyn Handler>) {
        self.next_handler = Some(next);
    }

    // FirstLineSupport can only handle PasswordResets
    fn handle(&self, request: &SupportRequest) {
        if matches!(request, SupportRequest::PasswordReset) {
            println!("FirstLineSupport: Resolved the PasswordReset request.");
        } else if let Some(ref next_handler) = self.next_handler {
            println!("FirstLineSupport: Cannot handle. Passing to next...");
            next_handler.handle(request);
        } else {
            println!("FirstLineSupport: Reached end of chain. Request unhandled.");
        }
    }
}

// ============= //
// B. Supervisor //
// ============= //

pub struct Supervisor {
    next_handler: Option<Box<dyn Handler>>,
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}

impl Supervisor {
    pub fn new() -> Self {
        Self { next_handler: None }
    }
}

impl Handler for Supervisor {
    fn set_next_handler(&mut self, next: Box<dyn Handler>) {
        self.next_handler = Some(next);
    }

    // Supervisor can only handle BillingIssues
    fn handle(&self, request: &SupportRequest) {
        if matches!(request, SupportRequest::BillingIssue) {
            println!("Supervisor: Resolved the BillingIssue request.");
        } else if let Some(ref next_handler) = self.next_handler {
            println!("Supervisor: Cannot handle. Passing to next...");
            next_handler.handle(request);
        } else {
            println!("Supervisor: Cannot handle. Reached end of chain. Request unhandled.");
        }
    }
}

// ================================== //
// 4. Construct the chain and execute //
// ================================== //

fn main() {
    let mut first = FirstLineSupport::new();
    let supervisor = Supervisor::new();

    // Link the chain together
    first.set_next_handler(Box::new(supervisor));

    // Test a request handled at the start
    println!("== Submitting Password Reset ==");
    first.handle(&SupportRequest::PasswordReset);

    // Test a request escalated down the chain
    println!("\n== Submitting Billing Issue ==");
    first.handle(&SupportRequest::BillingIssue);

    // Test an unhandled request
    println!("\n== Submitting Corporate Contract ==");
    first.handle(&SupportRequest::CorporateContract);
}
