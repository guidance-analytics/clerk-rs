/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

/// OrganizationInvitation : An organization invitation

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationInvitation {
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// String representing the object's type. Objects of the same type share the same value.
	#[serde(rename = "object", skip_serializing_if = "Option::is_none")]
	pub object: Option<Object>,
	#[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
	pub email_address: Option<String>,
	#[serde(rename = "role", skip_serializing_if = "Option::is_none")]
	pub role: Option<Role>,
	#[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
	pub organization_id: Option<String>,
	#[serde(rename = "status", skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
	pub public_metadata: Option<serde_json::Value>,
	#[serde(rename = "private_metadata", skip_serializing_if = "Option::is_none")]
	pub private_metadata: Option<serde_json::Value>,
	/// Unix timestamp of creation.
	#[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
	pub created_at: Option<i64>,
	/// Unix timestamp of last update.
	#[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
	pub updated_at: Option<i64>,
}

impl OrganizationInvitation {
	/// An organization invitation
	pub fn new() -> OrganizationInvitation {
		OrganizationInvitation {
			id: None,
			object: None,
			email_address: None,
			role: None,
			organization_id: None,
			status: None,
			public_metadata: None,
			private_metadata: None,
			created_at: None,
			updated_at: None,
		}
	}
}

/// String representing the object's type. Objects of the same type share the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "organization_invitation")]
	OrganizationInvitation,
}

impl Default for Object {
	fn default() -> Object {
		Self::OrganizationInvitation
	}
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
	#[serde(rename = "admin")]
	Admin,
	#[serde(rename = "basic_member")]
	BasicMember,
}

impl Default for Role {
	fn default() -> Role {
		Self::Admin
	}
}
