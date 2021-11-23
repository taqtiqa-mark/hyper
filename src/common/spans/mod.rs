trait Spans {
    #[cfg_attr(feature = "layers",
        instrument( level = "trace",
                    skip(buf, ctx),
                    fields( otel.name           = "Client::parse",
                            otel.kind           = ?SpanKind::Client,
                            otel.status_code    = ?opentelemetry::trace::StatusCode::Unset,
                            otel.status_message = tracing::field::Empty,
                            // OTel required (HTTP)
                            http.host     = tracing::field::Empty,
                            http.method   = tracing::field::Empty,
                            http.scheme   = tracing::field::Empty,
                            http.target   = tracing::field::Empty,
                            http.url      = tracing::field::Empty,
                            net.peer.ip   = tracing::field::Empty,
                            net.peer.name = tracing::field::Empty,
                            net.peer.port = tracing::field::Empty,
                            // OTel optional (HTTP)
                            http.flavor                 = tracing::field::Empty,
                            http.request_content_length = tracing::field::Empty,
                            // OTel optional (General)
                            net.transport = "IP.TCP",
                        )
                    )
                )]
    fn parse(buf: &mut BytesMut, ctx: ParseContext<'_>) -> ParseResult<StatusCode>;
}