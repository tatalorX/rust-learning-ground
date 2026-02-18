// Exercise 061: Generic Enums
//
// Generic enums can represent data structures with multiple variants,
// each potentially holding different generic types.
//
// Learning Objectives:
// - Define generic enums
// - Use generic enums to represent different states
// - Implement methods on generic enums
//
// Your task: Implement and use generic enums for various scenarios.

/// A generic Result-like enum for operations that can succeed or fail.
enum OperationResult<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> OperationResult<T, E> {
    /// Returns true if this is a Success.
    fn is_success(&self) -> bool {
        // TODO: Return true if this is Success, false otherwise
        todo!()
    }
    
    /// Returns true if this is a Failure.
    fn is_failure(&self) -> bool {
        // TODO: Return true if this is Failure, false otherwise
        todo!()
    }
    
    /// Maps a Success value using the provided function.
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> OperationResult<U, E> {
        // TODO: Apply f to the Success value, or pass through Failure
        todo!()
    }
    
    /// Returns the contained Success value or a default.
    fn unwrap_or(self, default: T) -> T {
        // TODO: Return the Success value or the default
        todo!()
    }
}

/// An enum representing a value that may or may not be present.
enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    /// Creates a new Just value.
    fn just(value: T) -> Self {
        // TODO: Create a Just variant with the given value
        todo!()
    }
    
    /// Creates a Nothing value.
    fn nothing() -> Self {
        // TODO: Return Nothing
        todo!()
    }
    
    /// Returns true if this is Just.
    fn is_just(&self) -> bool {
        // TODO: Return true if this is Just
        todo!()
    }
    
    /// Maps the contained value if present.
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        // TODO: Apply f if Just, or return Nothing
        todo!()
    }
    
    /// Returns the contained value or a default.
    fn unwrap_or(self, default: T) -> T {
        // TODO: Return the contained value or default
        todo!()
    }
}

/// A state machine enum with generic data for each state.
enum State<S, R, F> {
    Starting(S),
    Running(R),
    Finished(F),
}

impl<S, R, F> State<S, R, F> {
    /// Returns the name of the current state as a string.
    fn state_name(&self) -> &'static str {
        // TODO: Return "Starting", "Running", or "Finished" based on variant
        todo!()
    }
    
    /// Transitions from Starting to Running.
    fn start(self, running_data: R) -> Self {
        // TODO: If currently Starting, transition to Running with running_data
        // Otherwise, return self unchanged
        todo!()
    }
}

/// Either type - holds one of two possible values.
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Returns true if this is Left.
    fn is_left(&self) -> bool {
        // TODO: Return true if this is Left
        todo!()
    }
    
    /// Returns true if this is Right.
    fn is_right(&self) -> bool {
        // TODO: Return true if this is Right
        todo!()
    }
    
    /// Returns the Left value or a default.
    fn left_or(self, default: L) -> L {
        // TODO: Return Left value or default
        todo!()
    }
}

fn main() {
    // Test OperationResult
    let success: OperationResult<i32, &str> = OperationResult::Success(42);
    let failure: OperationResult<i32, &str> = OperationResult::Failure("error");
    
    println!("OperationResult tests:");
    println!("  Success is_success: {}", success.is_success());
    println!("  Failure is_failure: {}", failure.is_failure());
    println!("  Success unwrap_or 0: {}", success.unwrap_or(0));
    println!("  Failure unwrap_or 0: {}", failure.unwrap_or(0));
    
    let doubled = success.map(|x| x * 2);
    println!("  Success doubled: {:?}", doubled.is_success());
    
    // Test Maybe
    println!("\nMaybe tests:");
    let just_five = Maybe::just(5);
    let nothing: Maybe<i32> = Maybe::nothing();
    
    println!("  Just(5) is_just: {}", just_five.is_just());
    println!("  Nothing is_just: {}", nothing.is_just());
    println!("  Just(5) * 2: {:?}", just_five.map(|x| x * 2).unwrap_or(0));
    println!("  Nothing or 10: {}", nothing.unwrap_or(10));
    
    // Test State
    println!("\nState tests:");
    let starting: State<&str, i32, bool> = State::Starting("initializing");
    println!("  Initial state: {}", starting.state_name());
    let running = starting.start(42);
    println!("  After start: {}", running.state_name());
    
    // Test Either
    println!("\nEither tests:");
    let left: Either<i32, &str> = Either::Left(42);
    let right: Either<i32, &str> = Either::Right("hello");
    println!("  Left is_left: {}, is_right: {}", left.is_left(), left.is_right());
    println!("  Right is_left: {}, is_right: {}", right.is_left(), right.is_right());
    println!("  Left or 0: {}", left.left_or(0));
    println!("  Right or 0: {}", right.left_or(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_result_success() {
        let r: OperationResult<i32, &str> = OperationResult::Success(42);
        assert!(r.is_success());
        assert!(!r.is_failure());
        assert_eq!(r.unwrap_or(0), 42);
    }

    #[test]
    fn test_operation_result_failure() {
        let r: OperationResult<i32, &str> = OperationResult::Failure("error");
        assert!(!r.is_success());
        assert!(r.is_failure());
        assert_eq!(r.unwrap_or(0), 0);
    }

    #[test]
    fn test_maybe_just() {
        let m = Maybe::just(10);
        assert!(m.is_just());
        assert_eq!(m.unwrap_or(0), 10);
    }

    #[test]
    fn test_maybe_nothing() {
        let m: Maybe<i32> = Maybe::nothing();
        assert!(!m.is_just());
        assert_eq!(m.unwrap_or(5), 5);
    }

    #[test]
    fn test_state_transitions() {
        let s: State<&str, i32, bool> = State::Starting("init");
        assert_eq!(s.state_name(), "Starting");
        let s2 = s.start(42);
        assert_eq!(s2.state_name(), "Running");
    }

    #[test]
    fn test_either_left() {
        let e: Either<i32, &str> = Either::Left(42);
        assert!(e.is_left());
        assert!(!e.is_right());
        assert_eq!(e.left_or(0), 42);
    }

    #[test]
    fn test_either_right() {
        let e: Either<i32, &str> = Either::Right("hello");
        assert!(!e.is_left());
        assert!(e.is_right());
        assert_eq!(e.left_or(0), 0);
    }
}
