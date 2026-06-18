//
// Copyright (c) 2026 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use std::str::FromStr;

use nu_engine::CallExt;
use nu_protocol::{
    engine::{Call, Command, EngineState, Stack},
    record, IntoValue, PipelineData, Record, ShellError, Signature, Span, SyntaxShape, Type, Value,
};
use zenoh::query::Selector as ZenohSelector;
use zenoh_protocol::core::{EndPoint, Locator as ZenohLocator};

use crate::signature_ext::SignatureExt;

#[derive(Clone)]
pub(crate) struct Locator;

impl Command for Locator {
    fn name(&self) -> &str {
        "zenoh parse locator"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("locator", SyntaxShape::String, "Zenoh locator to parse")
            .input_output_type(Type::Nothing, Type::record())
            .zenoh_category()
    }

    fn description(&self) -> &str {
        "Parse a Zenoh locator"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let locator = call.req::<String>(engine_state, stack, 0)?;
        let locator = ZenohLocator::from_str(&locator).map_err(|err| {
            nu_protocol::LabeledError::new("Invalid Zenoh locator")
                .with_label(err.to_string(), call.arguments_span())
        })?;

        Ok(PipelineData::Value(
            locator_to_value(&locator, call.head),
            None,
        ))
    }
}

#[derive(Clone)]
pub(crate) struct Endpoint;

impl Command for Endpoint {
    fn name(&self) -> &str {
        "zenoh parse endpoint"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("endpoint", SyntaxShape::String, "Zenoh endpoint to parse")
            .input_output_type(Type::Nothing, Type::record())
            .zenoh_category()
    }

    fn description(&self) -> &str {
        "Parse a Zenoh endpoint"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let endpoint = call.req::<String>(engine_state, stack, 0)?;
        let endpoint = EndPoint::from_str(&endpoint).map_err(|err| {
            nu_protocol::LabeledError::new("Invalid Zenoh endpoint")
                .with_label(err.to_string(), call.arguments_span())
        })?;

        Ok(PipelineData::Value(
            endpoint_to_value(&endpoint, call.head),
            None,
        ))
    }
}

#[derive(Clone)]
pub(crate) struct Selector;

impl Command for Selector {
    fn name(&self) -> &str {
        "zenoh parse selector"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("selector", SyntaxShape::String, "Zenoh selector to parse")
            .input_output_type(Type::Nothing, Type::record())
            .zenoh_category()
    }

    fn description(&self) -> &str {
        "Parse a Zenoh selector"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let selector = call.req::<String>(engine_state, stack, 0)?;
        let selector = ZenohSelector::from_str(&selector).map_err(|err| {
            nu_protocol::LabeledError::new("Invalid Zenoh selector")
                .with_label(err.to_string(), call.arguments_span())
        })?;

        Ok(PipelineData::Value(
            selector_to_value(&selector, call.head),
            None,
        ))
    }
}

fn locator_to_value(locator: &ZenohLocator, span: Span) -> Value {
    record!(
        "protocol" => locator.protocol().as_str().into_value(span),
        "address" => locator.address().as_str().into_value(span),
        "metadata" => parameter_record_to_value(locator.metadata().iter(), span),
    )
    .into_value(span)
}

fn endpoint_to_value(endpoint: &EndPoint, span: Span) -> Value {
    record!(
        "protocol" => endpoint.protocol().as_str().into_value(span),
        "address" => endpoint.address().as_str().into_value(span),
        "metadata" => parameter_record_to_value(endpoint.metadata().iter(), span),
        "config" => parameter_record_to_value(endpoint.config().iter(), span),
    )
    .into_value(span)
}

fn selector_to_value(selector: &ZenohSelector<'_>, span: Span) -> Value {
    record!(
        "keyexpr" => selector.key_expr().as_str().into_value(span),
        "parameters" => parameter_table_to_value(selector.parameters().iter(), span),
    )
    .into_value(span)
}

fn parameter_record_to_value<'a>(
    parameters: impl Iterator<Item = (&'a str, &'a str)>,
    span: Span,
) -> Value {
    Record::from_iter(parameters.map(|(key, value)| (key.to_string(), value.into_value(span))))
        .into_value(span)
}

fn parameter_table_to_value<'a>(
    parameters: impl Iterator<Item = (&'a str, &'a str)>,
    span: Span,
) -> Value {
    parameters
        .map(|(key, value)| {
            record!(
                "key" => key.into_value(span),
                "value" => value.into_value(span),
            )
            .into_value(span)
        })
        .collect::<Vec<_>>()
        .into_value(span)
}
