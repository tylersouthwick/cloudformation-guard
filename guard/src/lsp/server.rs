use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tokio::io::BufReader;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

//use cfn_guard::rules_file;
use cfn_guard::rules::exprs::RuleFile;

#[derive(thiserror::Error, Debug)]
pub enum LspError {

}

#[derive(Debug)]
struct Backend {
    client: Client,
}

const LEGEND_TYPE: &[SemanticTokenType] = &[
    SemanticTokenType::FUNCTION,
];

struct TextDocumentItem {
    uri: Url,
    text: String,
    version: i32,
}

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
            self.client
                .log_message(MessageType::ERROR, "on_change")
                .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    legend: SemanticTokensLegend {
                        token_types: LEGEND_TYPE.into(),
                        token_modifiers: vec![]
                    },
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                    .. SemanticTokensOptions::default()
                })),
                definition_provider: Some(OneOf::Left(true)),
                .. ServerCapabilities::default()
            },
            .. InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::ERROR, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::ERROR, format!("file opened: {}", params.text_document.uri))
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        })
        .await
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        self.client
            .log_message(MessageType::ERROR, format!("looking for semantic tokens: {}", params.text_document.uri.to_string()))
            .await;
        /*
        let mut content = String::new();
        let file = params.text_document.uri;
        let mut reader = BufReader::new(File::open(file.path()).await?);
        reader.read_to_string(&mut content).await?;
        */

        //rules_file()
        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        self.client
            .log_message(MessageType::ERROR, &format!("goto definition {uri}, "))
            .await;

        let contents = tokio::fs::read_to_string(uri.to_file_path().unwrap()).await.unwrap();

        //let result = parse_template_and_call_gen(&template_contents, writer);
        //print_rules(result, writer)?;
        let result = crate::rules::parser::rules_file(contents.as_str().into());
        self.client
            .log_message(MessageType::ERROR, &format!("{:?}, ", result))
            .await;

        
        Ok(None)
    }
}

fn find_location(rule_file : Option<RuleFile>) -> Option<Span> {
    None
}

pub async fn run(listen : bool) {
    /*
    let stream = if listen {
        let listener = TcpListener::bind("127.0.0.1:9257").await.unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        stream
    } else {
        TcpStream::connect("127.0.0.1:9257").await.unwrap()
    };
    let (read, write) = tokio::io::split(stream);
    */
    let read = tokio::io::stdin();
    let write = tokio::io::stdout();
    
    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(read, write, socket).serve(service).await;
}
