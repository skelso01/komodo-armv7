use derive_builder::Builder;
use partial_derive2::{Diff, MaybeNone, Partial, PartialDiff};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumDiscriminants, EnumString};
use typeshare::typeshare;

use super::{
  MergePartial,
  resource::{AddFilters, Resource, ResourceListItem, ResourceQuery},
};

#[cfg(feature = "utoipa")]
#[derive(utoipa::ToSchema)]
#[schema(as = Builder)]
pub struct BuilderSchema(
  #[schema(inline)]
  pub  Resource<BuilderConfig, crate::entities::NoData>,
);

#[typeshare]
pub type Builder = Resource<BuilderConfig, ()>;

#[typeshare]
pub type BuilderListItem = ResourceListItem<BuilderListItemInfo>;

#[typeshare(serialized_as = "Partial<BuilderConfig>")]
pub type _PartialBuilderConfig = PartialBuilderConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BuilderListItemInfo {
  /// 'Url' or 'Server'
  pub builder_type: String,
  /// If 'Url': null
  /// If 'Server': the server id
  pub instance_type: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(BuilderConfigVariant))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
  not(feature = "utoipa"),
  strum_discriminants(derive(
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    AsRefStr
  ))
)]
#[cfg_attr(
  feature = "utoipa",
  strum_discriminants(derive(
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    AsRefStr,
    utoipa::ToSchema
  ))
)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::large_enum_variant)]
pub enum BuilderConfig {
  /// Use a Periphery address as a Builder.
  Url(UrlBuilderConfig),

  /// Use a connected server as a Builder.
  Server(ServerBuilderConfig),
}

impl Default for BuilderConfig {
  fn default() -> Self {
    Self::Url(Default::default())
  }
}

/// Partial representation of [BuilderConfig]
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(PartialBuilderConfigVariant))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
  not(feature = "utoipa"),
  strum_discriminants(derive(
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    AsRefStr
  ))
)]
#[cfg_attr(
  feature = "utoipa",
  strum_discriminants(derive(
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    AsRefStr,
    utoipa::ToSchema
  ))
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "type", content = "params")]
#[allow(clippy::large_enum_variant)]
pub enum PartialBuilderConfig {
  Url(#[serde(default)] _PartialUrlBuilderConfig),
  Server(#[serde(default)] _PartialServerBuilderConfig),
}

impl Default for PartialBuilderConfig {
  fn default() -> Self {
    Self::Url(Default::default())
  }
}

impl MaybeNone for PartialBuilderConfig {
  fn is_none(&self) -> bool {
    match self {
      PartialBuilderConfig::Url(config) => config.is_none(),
      PartialBuilderConfig::Server(config) => config.is_none(),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuilderConfigDiff {
  Url(UrlBuilderConfigDiff),
  Server(ServerBuilderConfigDiff),
}

impl From<BuilderConfigDiff> for PartialBuilderConfig {
  fn from(value: BuilderConfigDiff) -> Self {
    match value {
      BuilderConfigDiff::Url(diff) => {
        PartialBuilderConfig::Url(diff.into())
      }
      BuilderConfigDiff::Server(diff) => {
        PartialBuilderConfig::Server(diff.into())
      }
    }
  }
}

impl Diff for BuilderConfigDiff {
  fn iter_field_diffs(
    &self,
  ) -> impl Iterator<Item = partial_derive2::FieldDiff> {
    match self {
      BuilderConfigDiff::Url(diff) => {
        diff.iter_field_diffs().collect::<Vec<_>>().into_iter()
      }
      BuilderConfigDiff::Server(diff) => {
        diff.iter_field_diffs().collect::<Vec<_>>().into_iter()
      }
    }
  }
}

impl PartialDiff<PartialBuilderConfig, BuilderConfigDiff>
  for BuilderConfig
{
  fn partial_diff(
    &self,
    partial: PartialBuilderConfig,
  ) -> BuilderConfigDiff {
    match self {
      BuilderConfig::Url(original) => match partial {
        PartialBuilderConfig::Url(partial) => {
          BuilderConfigDiff::Url(original.partial_diff(partial))
        }
        PartialBuilderConfig::Server(partial) => {
          let default = ServerBuilderConfig::default();
          BuilderConfigDiff::Server(default.partial_diff(partial))
        }
      },
      BuilderConfig::Server(original) => match partial {
        PartialBuilderConfig::Server(partial) => {
          BuilderConfigDiff::Server(original.partial_diff(partial))
        }
        PartialBuilderConfig::Url(partial) => {
          let default = UrlBuilderConfig::default();
          BuilderConfigDiff::Url(default.partial_diff(partial))
        }
      },
    }
  }
}

impl MaybeNone for BuilderConfigDiff {
  fn is_none(&self) -> bool {
    match self {
      BuilderConfigDiff::Url(config) => config.is_none(),
      BuilderConfigDiff::Server(config) => config.is_none(),
    }
  }
}

impl From<PartialBuilderConfig> for BuilderConfig {
  fn from(value: PartialBuilderConfig) -> BuilderConfig {
    match value {
      PartialBuilderConfig::Url(server) => {
        BuilderConfig::Url(server.into())
      }
      PartialBuilderConfig::Server(server) => {
        BuilderConfig::Server(server.into())
      }
    }
  }
}

impl From<BuilderConfig> for PartialBuilderConfig {
  fn from(value: BuilderConfig) -> Self {
    match value {
      BuilderConfig::Url(config) => {
        PartialBuilderConfig::Url(config.into())
      }
      BuilderConfig::Server(config) => {
        PartialBuilderConfig::Server(config.into())
      }
    }
  }
}

impl MergePartial for BuilderConfig {
  type Partial = PartialBuilderConfig;
  fn merge_partial(
    self,
    partial: PartialBuilderConfig,
  ) -> BuilderConfig {
    match partial {
      PartialBuilderConfig::Url(partial) => match self {
        BuilderConfig::Url(config) => {
          let config = UrlBuilderConfig {
            address: partial.address.unwrap_or(config.address),
            periphery_public_key: partial
              .periphery_public_key
              .unwrap_or(config.periphery_public_key),
            insecure_tls: partial
              .insecure_tls
              .unwrap_or(config.insecure_tls),
            passkey: partial.passkey.unwrap_or(config.passkey),
          };
          BuilderConfig::Url(config)
        }
        _ => BuilderConfig::Url(partial.into()),
      },
      PartialBuilderConfig::Server(partial) => match self {
        BuilderConfig::Server(config) => {
          let config = ServerBuilderConfig {
            server_id: partial.server_id.unwrap_or(config.server_id),
          };
          BuilderConfig::Server(config)
        }
        _ => BuilderConfig::Server(partial.into()),
      },
    }
  }
}

#[typeshare(serialized_as = "Partial<UrlBuilderConfig>")]
pub type _PartialUrlBuilderConfig = PartialUrlBuilderConfig;

/// Configuration for a Komodo Url Builder.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(
  feature = "schemars",
  partial_derive(schemars::JsonSchema)
)]
#[diff_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct UrlBuilderConfig {
  /// The address of the Periphery agent
  #[serde(default = "default_address")]
  #[builder(default = default_address())]
  #[partial_default(default_address())]
  pub address: String,
  /// An expected public key associated with Periphery private key.
  /// If empty, doesn't validate Periphery public key.
  #[serde(default)]
  #[builder(default)]
  pub periphery_public_key: String,
  /// Whether to validate the Periphery tls certificates.
  #[serde(default = "default_insecure_tls")]
  #[builder(default = default_insecure_tls())]
  #[partial_default(default_insecure_tls())]
  pub insecure_tls: bool,
  /// Deprecated. Use private / public keys instead.
  /// An optional override passkey to use
  /// to authenticate with periphery agent.
  /// If this is empty, will use passkey in core config.
  #[serde(default)]
  #[builder(default)]
  pub passkey: String,
}

fn default_address() -> String {
  String::from("https://periphery:8120")
}

fn default_insecure_tls() -> bool {
  true
}

impl Default for UrlBuilderConfig {
  fn default() -> Self {
    Self {
      address: default_address(),
      periphery_public_key: Default::default(),
      insecure_tls: default_insecure_tls(),
      passkey: Default::default(),
    }
  }
}

impl UrlBuilderConfig {
  pub fn builder() -> UrlBuilderConfigBuilder {
    UrlBuilderConfigBuilder::default()
  }
}

#[cfg(feature = "utoipa")]
impl utoipa::PartialSchema for PartialUrlBuilderConfig {
  fn schema()
  -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
    utoipa::schema!(#[inline] std::collections::HashMap<String, serde_json::Value>).into()
  }
}

#[cfg(feature = "utoipa")]
impl utoipa::ToSchema for PartialUrlBuilderConfig {}

#[typeshare(serialized_as = "Partial<ServerBuilderConfig>")]
pub type _PartialServerBuilderConfig = PartialServerBuilderConfig;

/// Configuration for a Komodo Server Builder.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Builder, Partial,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(
  feature = "schemars",
  partial_derive(schemars::JsonSchema)
)]
#[diff_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct ServerBuilderConfig {
  /// The server id of the builder
  #[serde(default, alias = "server")]
  #[partial_attr(serde(alias = "server"))]
  #[cfg_attr(
    feature = "schemars",
    partial_attr(schemars(rename = "server"))
  )]
  pub server_id: String,
}

impl ServerBuilderConfig {
  pub fn builder() -> ServerBuilderConfigBuilder {
    ServerBuilderConfigBuilder::default()
  }
}

#[cfg(feature = "utoipa")]
impl utoipa::PartialSchema for PartialServerBuilderConfig {
  fn schema()
  -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
    utoipa::schema!(#[inline] std::collections::HashMap<String, serde_json::Value>).into()
  }
}

#[cfg(feature = "utoipa")]
impl utoipa::ToSchema for PartialServerBuilderConfig {}

#[typeshare]
pub type BuilderQuery = ResourceQuery<BuilderQuerySpecifics>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BuilderQuerySpecifics {}

impl AddFilters for BuilderQuerySpecifics {}
