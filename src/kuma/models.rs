use crate::util::{DeserializeBoolLenient, DeserializeNumberLenient};
use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;
use serde_inline_default::serde_inline_default;
use serde_with::{serde_as, skip_serializing_none};
use std::collections::HashMap;
use strum::EnumString;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DnsResolverType {
    A,
    AAAA,
    CAA,
    CNAME,
    MX,
    NS,
    PTR,
    SOA,
    SRV,
    TXT,
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "camelCase")]
pub enum Event {
    Connect,
    Disconnect,
    MonitorList,
    NotificationList,
    ProxyList,
    StatusPageList,
    HeartbeatList,
    ImportantHeartbeatList,
    AvgPing,
    Uptime,
    Heartbeat,
    Info,
    CertInfo,
    DockerHostList,
    AutoLogin,
    InitServerTimezone,
    MaintenanceList,
    ApiKeyList,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MonitorType {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "port")]
    Port,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "keyword")]
    Keyword,
    #[serde(rename = "json-query")]
    JsonQuery,
    #[serde(rename = "grpc-keyword")]
    GrpcKeyword,
    #[serde(rename = "dns")]
    Dns,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "real-browser")]
    RealBrowser,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "steam")]
    Steam,
    #[serde(rename = "gamedig")]
    GameDig,
    #[serde(rename = "mqtt")]
    Mqtt,
    #[serde(rename = "kafka-producer")]
    KafkaProducer,
    #[serde(rename = "sqlserver")]
    SqlServer,
    #[serde(rename = "postgres")]
    Postgres,
    #[serde(rename = "mysql")]
    Mysql,
    #[serde(rename = "mongodb")]
    Mongodb,
    #[serde(rename = "radius")]
    Radius,
    #[serde(rename = "redis")]
    Redis,
    #[serde(rename = "tailscale-ping")]
    TailscalePing,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum KafkaProducerSaslOption {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "plain")]
    Plain {
        #[serde(rename = "username")]
        username: Option<String>,
        #[serde(rename = "password")]
        password: Option<String>,
    },
    #[serde(rename = "scram-sha-256")]
    ScramSha256 {
        #[serde(rename = "username")]
        username: Option<String>,
        #[serde(rename = "password")]
        password: Option<String>,
    },
    #[serde(rename = "scram-sha-512")]
    ScramSha512 {
        #[serde(rename = "username")]
        username: Option<String>,
        #[serde(rename = "password")]
        password: Option<String>,
    },
    #[serde(rename = "aws")]
    AWS {
        #[serde(rename = "authorizationIdentity")]
        #[serde(alias = "authorization_identity")]
        authorization_identity: Option<String>,
        #[serde(rename = "accessKeyId")]
        #[serde(alias = "name")]
        access_key_id: Option<String>,
        #[serde(rename = "secretAccessKey")]
        #[serde(alias = "name")]
        secret_access_key: Option<String>,
        #[serde(alias = "name")]
        #[serde(rename = "sessionToken")]
        session_token: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

#[serde_inline_default]
#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorCommon {
    #[serde(rename = "id")]
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "interval")]
    #[serde_inline_default(Some(60))]
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub interval: Option<i32>,
    #[serde(rename = "active")]
    #[serde_inline_default(None)]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    pub active: Option<bool>,
    #[serde(rename = "maxretries")]
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub max_retries: Option<i32>,
    #[serde(rename = "retryInterval")]
    #[serde_inline_default(Some(60))]
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub retry_interval: Option<i32>,
    #[serde(rename = "upsideDown")]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    pub upside_down: Option<bool>,
    #[serde(rename = "parent")]
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub parent: Option<i32>,
    #[serde(rename = "group")]
    #[serde(skip_serializing)]
    pub group: Option<String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(rename = "notificationIDList")]
    pub notification_id_list: Option<HashMap<String, bool>>,
    #[serde(rename = "accepted_statuscodes")]
    #[serde_inline_default(vec!["200-299".to_owned()])]
    pub accepted_statuscodes: Vec<String>,
}

#[serde_inline_default]
#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorHttp {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "timeout")]
    #[serde_inline_default(Some(48))]
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub timeout: Option<i32>,
    #[serde(rename = "resendInterval")]
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub resend_interval: Option<i32>,
    #[serde(rename = "expiryNotification")]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    pub expiry_notification: Option<bool>,
    #[serde(rename = "ignoreTls")]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    pub ignore_tls: Option<bool>,
    #[serde(rename = "maxredirects")]
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub max_redirects: Option<i32>,
    #[serde(rename = "proxyId")]
    pub proxy_id: Option<String>,
    #[serde(rename = "method")]
    #[serde_inline_default(Some(HttpMethod::GET))]
    pub method: Option<HttpMethod>,
    #[serde(rename = "httpBodyEncoding")]
    pub http_body_encoding: Option<String>,
    #[serde(rename = "body")]
    pub body: Option<String>,
    #[serde(rename = "headers")]
    pub headers: Option<String>,
    #[serde(rename = "authMethod")]
    pub auth_method: Option<String>,
    #[serde(rename = "tlsCert")]
    pub tls_cert: Option<String>,
    #[serde(rename = "tlsKey")]
    pub tls_key: Option<String>,
    #[serde(rename = "tlsCa")]
    pub tls_ca: Option<String>,
    #[serde(rename = "oauth_auth_method")]
    pub oauth_auth_method: Option<String>,
    #[serde(rename = "oauth_client_id")]
    pub oauth_client_id: Option<String>,
    #[serde(rename = "oauth_token_url")]
    pub oauth_token_url: Option<String>,
    #[serde(rename = "oauth_client_secret")]
    pub oauth_client_secret: Option<String>,
    #[serde(rename = "oauth_scopes")]
    pub oauth_scopes: Option<String>,
    #[serde(rename = "basic_auth_user")]
    pub basic_auth_user: Option<String>,
    #[serde(rename = "basic_auth_pass")]
    pub basic_auth_pass: Option<String>,
    #[serde(rename = "authDomain")]
    pub auth_domain: Option<String>,
    #[serde(rename = "authWorkstation")]
    pub auth_workstation: Option<String>,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorPort {
    #[serde(rename = "hostname")]
    hostname: String,
    #[serde(rename = "port")]
    port: String,
}

#[serde_inline_default]
#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorPing {
    #[serde(rename = "hostname")]
    hostname: String,
    #[serde(rename = "packetSize")]
    #[serde_inline_default(56)]
    packet_size: i32,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorKeyword {
    #[serde(rename = "keyword")]
    keyword: String,
    #[serde(rename = "invertKeyword")]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    invert_keyword: Option<bool>,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorJsonQuery {
    #[serde(rename = "jsonPath")]
    json_path: Option<String>,
    #[serde(rename = "expectedValue")]
    expected_value: Option<String>,
}

#[serde_inline_default]
#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorGrpcKeyword {
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    invert_keyword: Option<bool>,
    #[serde(rename = "grpcUrl")]
    grpc_url: String,
    #[serde(rename = "maxredirects")]
    #[serde_inline_default(10)]
    max_redirects: i32,
    #[serde(rename = "grpcEnableTls")]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    grpc_enable_tls: Option<bool>,
    #[serde(rename = "grpcServiceName")]
    grpc_service_name: String,
    #[serde(rename = "grpcMethod")]
    grpc_method: String,
    #[serde(rename = "grpcProtobuf")]
    grpc_protobuf: Option<String>,
    #[serde(rename = "grpcBody")]
    grpc_body: Option<String>,
    #[serde(rename = "grpcMetadata")]
    grpc_metadata: Option<String>,
}

#[serde_inline_default]
#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorDns {
    #[serde(rename = "hostname")]
    hostname: String,
    #[serde(rename = "dns_resolve_server")]
    #[serde_inline_default("1.1.1.1".to_owned())]
    dns_resolve_server: String,
    #[serde(rename = "port")]
    port: String,
    #[serde(rename = "dns_resolve_type")]
    #[serde_inline_default(DnsResolverType::A)]
    dns_resolve_type: DnsResolverType,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorDocker {
    #[serde(rename = "docker_container")]
    docker_container: String,
    #[serde(rename = "docker_host")]
    docker_host: String,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorRealBrowser {
    #[serde(rename = "url")]
    url: String,
    #[serde(rename = "remoteBrowsersToggle")]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    remote_browsers_toggle: Option<bool>,
    #[serde(rename = "remote_browser")]
    remote_browser: Option<String>,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorPush {
    #[serde(rename = "pushURL")]
    push_url: Option<String>,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorSteam {
    #[serde(rename = "hostname")]
    hostname: String,
    #[serde(rename = "port")]
    port: String,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorGameDig {
    #[serde(rename = "game")]
    game: String,
    #[serde(rename = "hostname")]
    hostname: String,
    #[serde(rename = "port")]
    port: String,
    #[serde(rename = "gamedigGivenPortOnly")]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    gamedig_given_port_only: Option<bool>,
}

#[serde_inline_default]
#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorMqtt {
    #[serde(rename = "hostname")]
    hostname: String,
    #[serde(rename = "port")]
    port: String,
    #[serde(rename = "mqttUsername")]
    mqtt_username: Option<String>,
    #[serde(rename = "mqttPassword")]
    mqtt_password: Option<String>,
    #[serde(rename = "mqttTopic")]
    mqtt_topic: String,
    #[serde(rename = "mqttCheckType")]
    mqtt_check_type: Option<String>,
    #[serde(rename = "mqttSuccessMessage")]
    mqtt_success_message: Option<String>,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorKafkaProducer {
    #[serde(rename = "kafkaProducerBrokers")]
    kafka_producer_brokers: Option<String>,
    #[serde(rename = "kafkaProducerTopic")]
    kafka_producer_topic: Option<String>,
    #[serde(rename = "kafkaProducerMessage")]
    kafka_producer_message: Option<String>,
    #[serde(rename = "kafkaProducerSsl")]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    kafka_producer_ssl: Option<bool>,
    #[serde(rename = "kafkaProducerAllowAutoTopicCreation")]
    #[serde_as(as = "Option<DeserializeBoolLenient>")]
    kafka_producer_allow_auto_topic_creation: Option<bool>,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorDatabase {
    #[serde(rename = "databaseConnectionString")]
    database_connection_string: Option<String>,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorRadius {
    #[serde(rename = "hostname")]
    hostname: Option<String>,
    #[serde(rename = "port")]
    port: Option<String>,
    #[serde(rename = "radiusUsername")]
    radius_username: String,
    #[serde(rename = "radiusPassword")]
    radius_password: String,
    #[serde(rename = "radiusSecret")]
    radius_secret: String,
    #[serde(rename = "radiusCalledStationId")]
    radius_called_station_id: String,
    #[serde(rename = "radiusCallingStationId")]
    radius_calling_station_id: String,
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MonitorTailscale {
    #[serde(rename = "hostname")]
    hostname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Monitor {
    #[serde(rename = "group")]
    Group {
        #[serde(flatten)]
        common: MonitorCommon,
    },
    #[serde(rename = "http")]
    Http {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        http: MonitorHttp,
    },
    #[serde(rename = "port")]
    Port {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        port: MonitorPort,
    },
    #[serde(rename = "ping")]
    Ping {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        ping: MonitorPing,
    },
    #[serde(rename = "keyword")]
    Keyword {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        http: MonitorHttp,
        #[serde(flatten)]
        keyword: MonitorKeyword,
    },
    #[serde(rename = "json-query")]
    JsonQuery {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        http: MonitorHttp,
        #[serde(flatten)]
        json_query: MonitorJsonQuery,
    },
    #[serde(rename = "grpc-keyword")]
    GrpcKeyword {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        keyword: MonitorKeyword,
        #[serde(flatten)]
        grpc_keyword: MonitorGrpcKeyword,
    },
    #[serde(rename = "dns")]
    Dns {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        dns: MonitorDns,
    },
    #[serde(rename = "docker")]
    Docker {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        docker: MonitorDocker,
    },
    #[serde(rename = "real-browser")]
    RealBrowser {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        real_browser: MonitorRealBrowser,
    },
    #[serde(rename = "push")]
    Push {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        push: MonitorPush,
    },
    #[serde(rename = "steam")]
    Steam {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        steam: MonitorSteam,
    },
    #[serde(rename = "gamedig")]
    GameDig {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        gamedig: MonitorGameDig,
    },
    #[serde(rename = "mqtt")]
    Mqtt {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        mqtt: MonitorMqtt,
    },
    #[serde(rename = "kafka-producer")]
    KafkaProducer {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        kafka_producer: MonitorKafkaProducer,
    },
    #[serde(rename = "sqlserver")]
    SqlServer {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        database: MonitorDatabase,
    },
    #[serde(rename = "postgres")]
    Postgres {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        database: MonitorDatabase,
    },
    #[serde(rename = "mysql")]
    Mysql {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        database: MonitorDatabase,
        #[serde(rename = "radiusPassword")]
        password: Option<String>,
    },
    #[serde(rename = "mongodb")]
    Mongodb {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        database: MonitorDatabase,
    },
    #[serde(rename = "radius")]
    Radius {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        radius: MonitorRadius,
    },
    #[serde(rename = "redis")]
    Redis {
        #[serde(flatten)]
        common: MonitorCommon,
        #[serde(flatten)]
        database: MonitorDatabase,
    },
    #[serde(rename = "tailscale-ping")]
    TailscalePing {
        #[serde(flatten)]
        common: MonitorCommon,
    },
}

impl Monitor {
    pub fn common(&self) -> &MonitorCommon {
        match self {
            Monitor::Group { common } => common,
            Monitor::Http { common, .. } => common,
            Monitor::Port { common, .. } => common,
            Monitor::Ping { common, .. } => common,
            Monitor::Keyword { common, .. } => common,
            Monitor::JsonQuery { common, .. } => common,
            Monitor::GrpcKeyword { common, .. } => common,
            Monitor::Dns { common, .. } => common,
            Monitor::Docker { common, .. } => common,
            Monitor::RealBrowser { common, .. } => common,
            Monitor::Push { common, .. } => common,
            Monitor::Steam { common, .. } => common,
            Monitor::GameDig { common, .. } => common,
            Monitor::Mqtt { common, .. } => common,
            Monitor::KafkaProducer { common, .. } => common,
            Monitor::SqlServer { common, .. } => common,
            Monitor::Postgres { common, .. } => common,
            Monitor::Mysql { common, .. } => common,
            Monitor::Mongodb { common, .. } => common,
            Monitor::Radius { common, .. } => common,
            Monitor::Redis { common, .. } => common,
            Monitor::TailscalePing { common, .. } => common,
        }
    }
    pub fn common_mut(&mut self) -> &mut MonitorCommon {
        match self {
            Monitor::Group { common } => common,
            Monitor::Http { common, .. } => common,
            Monitor::Port { common, .. } => common,
            Monitor::Ping { common, .. } => common,
            Monitor::Keyword { common, .. } => common,
            Monitor::JsonQuery { common, .. } => common,
            Monitor::GrpcKeyword { common, .. } => common,
            Monitor::Dns { common, .. } => common,
            Monitor::Docker { common, .. } => common,
            Monitor::RealBrowser { common, .. } => common,
            Monitor::Push { common, .. } => common,
            Monitor::Steam { common, .. } => common,
            Monitor::GameDig { common, .. } => common,
            Monitor::Mqtt { common, .. } => common,
            Monitor::KafkaProducer { common, .. } => common,
            Monitor::SqlServer { common, .. } => common,
            Monitor::Postgres { common, .. } => common,
            Monitor::Mysql { common, .. } => common,
            Monitor::Mongodb { common, .. } => common,
            Monitor::Radius { common, .. } => common,
            Monitor::Redis { common, .. } => common,
            Monitor::TailscalePing { common, .. } => common,
        }
    }

    pub fn monitor_type(&self) -> MonitorType {
        match self {
            Monitor::Group { .. } => MonitorType::Group,
            Monitor::Http { .. } => MonitorType::Http,
            Monitor::Port { .. } => MonitorType::Port,
            Monitor::Ping { .. } => MonitorType::Ping,
            Monitor::Keyword { .. } => MonitorType::Keyword,
            Monitor::JsonQuery { .. } => MonitorType::JsonQuery,
            Monitor::GrpcKeyword { .. } => MonitorType::GrpcKeyword,
            Monitor::Dns { .. } => MonitorType::Dns,
            Monitor::Docker { .. } => MonitorType::Docker,
            Monitor::RealBrowser { .. } => MonitorType::RealBrowser,
            Monitor::Push { .. } => MonitorType::Push,
            Monitor::Steam { .. } => MonitorType::Steam,
            Monitor::GameDig { .. } => MonitorType::GameDig,
            Monitor::Mqtt { .. } => MonitorType::Mqtt,
            Monitor::KafkaProducer { .. } => MonitorType::KafkaProducer,
            Monitor::SqlServer { .. } => MonitorType::SqlServer,
            Monitor::Postgres { .. } => MonitorType::Postgres,
            Monitor::Mysql { .. } => MonitorType::Mysql,
            Monitor::Mongodb { .. } => MonitorType::Mongodb,
            Monitor::Radius { .. } => MonitorType::Radius,
            Monitor::Redis { .. } => MonitorType::Redis,
            Monitor::TailscalePing { .. } => MonitorType::TailscalePing,
        }
    }
}

#[skip_serializing_none]
#[serde_alias(SnakeCase)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Tag {
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub id: Option<i32>,
    pub name: String,
    pub color: String,
    #[serde_as(as = "Option<DeserializeNumberLenient>")]
    pub tag_id: Option<i32>,
    pub value: Option<String>,
}

// #[derive(Debug, Serialize, Deserialize, Default)]
// pub struct MonitorList {
//     monitors: Vec<Monitor>,
// }
pub type MonitorList = HashMap<String, Monitor>;

// #[serde_alias(SnakeCase)]
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct TagList {
//     pub tags: Vec<Tag>,
// }
pub type TagList = Vec<Tag>;
