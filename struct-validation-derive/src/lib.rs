use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Error};

/// Importing `ValidationError` from the `struct_validation_core` crate.
/// This is used to annotate validation errors with field-specific information.
#[allow(unused_imports)]
use struct_validation_core::ValidationError;

/// Procedural macro to automatically implement the `Validate` trait for structs.
///
/// This macro generates an implementation of the `Validate` trait for the annotated struct.
/// It iterates over each named field in the struct, invokes the `validate` method on each field,
/// prefixes any resulting `ValidationError` with the field name, and collects all errors into
/// a single `Vec<ValidationError>`.
///
/// # Constraints
///
/// - The macro can only be derived for structs with **named fields**.
/// - Each field in the struct must implement the `Validate` trait.
///
/// # Examples
///
/// ```rust
/// use struct_validation_core::{Validate, ValidationError, validate};
/// use struct_validation_derive::Validate;
///
/// struct NonEmptyString(String);
/// 
/// impl Validate for NonEmptyString {
///     fn validate(&self) -> Vec<ValidationError> {
///         let mut errors = Vec::new();
///         if self.0.is_empty() {
///             errors.push(ValidationError::new("String", "must not be empty"));
///         }
///         errors
///     }
/// }
/// impl From<String> for NonEmptyString {
///     fn from(value: String) -> Self {
///        Self(value)
///     }
/// }
/// 
/// #[derive(Validate)]
/// struct User {
///     username: NonEmptyString,
///     email: NonEmptyString,
/// }
///
///
/// fn main() {
///     let user = User {
///         username: "".to_string().into(),
///         email: "invalidemail.com".to_string().into(),
///     };
///
///     let errors = user.validate();
///
///     for error in errors {
///         println!("Error in {}: {}", error.field, error.message);
///     }
/// }
/// ```
///
/// **Output:**
/// ```text
/// Error in username: must not be empty
/// Error in email: must not be empty
/// ```
#[proc_macro_derive(Validate)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    // Parse the input token stream as a Rust struct
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let struct_name = &input.ident;

    // Ensure the input is a struct with named fields
    let fields = if let Data::Struct(data) = &input.data {
        match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                // Emit a compile error if not a struct with named fields
                return Error::new_spanned(
                    struct_name,
                    "Validate can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        // Emit a compile error if not a struct
        return Error::new_spanned(
            struct_name,
            "Validate can only be derived for structs",
        )
        .to_compile_error()
        .into();
    };

    // Generate validation code for each field, ensuring each implements Validate
    let validator_iters = fields.iter().map(|field| {
        // Extract the field name as an identifier
        let field_name = &field.ident;
        // Convert the field name to a string for error prefixing
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            self.#field_name.validate()
                .into_iter()
                .map(|mut e| { e.add_prefix(#field_name_str); e })
        }
    });

    // Chain all iterators or use an empty iterator if no fields are present
    let stream = validator_iters.reduce(|acc, stream| {
        quote! {
            #acc.chain(#stream)
        }
    }).unwrap_or_else(|| quote! { std::iter::empty() });

    // Generate the final implementation of Validate for the struct
    let expanded = quote! {
        impl Validate for #struct_name {
            fn validate(&self) -> Vec<struct_validation_core::ValidationError> {
                #stream.collect()
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}
