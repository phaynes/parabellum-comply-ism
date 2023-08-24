
// catalog ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct catalog {
	#[serde(rename = "catalog")]
	pub catalog: OscalcatalogcatalogASSEMBLY,
}


// OscalcatalogcatalogASSEMBLY is : A structured, organized collection of control information.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcatalogcatalogASSEMBLY {
	#[serde(rename = "uuid")]
	pub uuid: UUIDDatatype,
	#[serde(rename = "metadata")]
	pub metadata: OscalmetadatametadataASSEMBLY,
	#[serde(rename = "param")]
	pub param: Vec<OscalcontrolcommonparameterASSEMBLY>,
	#[serde(rename = "control")]
	pub control: Vec<OscalcatalogcontrolASSEMBLY>,
	#[serde(rename = "group")]
	pub group: Vec<OscalcataloggroupASSEMBLY>,
	#[serde(rename = "back-matter")]
	pub backmatter: OscalmetadatabackmatterASSEMBLY,
}


// Title is : A name given to the group, which may be used by a tool for display and navigation.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Title {
	#[serde(flatten)]
	pub markup_line_datatype: MarkupLineDatatype,
}


// OscalcataloggroupASSEMBLY is : A group of controls, or of groups of controls.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcataloggroupASSEMBLY {
	#[serde(rename = "id")]
	pub id: Option<String>,
	#[serde(rename = "class")]
	pub class: Option<String>,
	#[serde(rename = "title")]
	pub title: Title,
	#[serde(rename = "param")]
	pub param: Vec<OscalcontrolcommonparameterASSEMBLY>,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "part")]
	pub part: Vec<OscalcontrolcommonpartASSEMBLY>,
	#[serde(rename = "group")]
	pub group: Vec<OscalcataloggroupASSEMBLY>,
	#[serde(rename = "control")]
	pub control: Vec<OscalcatalogcontrolASSEMBLY>,
}


// OscalcatalogcontrolASSEMBLY is : A structured object representing a requirement or guideline, which when implemented will reduce an aspect of risk related to an information system and its information.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcatalogcontrolASSEMBLY {
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "class")]
	pub class: Option<String>,
	#[serde(rename = "title")]
	pub title: Title,
	#[serde(rename = "param")]
	pub param: Vec<OscalcontrolcommonparameterASSEMBLY>,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "part")]
	pub part: Vec<OscalcontrolcommonpartASSEMBLY>,
	#[serde(rename = "control")]
	pub control: Vec<OscalcatalogcontrolASSEMBLY>,
}


// OscalcontrolcommonpartASSEMBLY is : An annotated, markup-based textual element of a control's or catalog group's definition, or a child of another part.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcontrolcommonpartASSEMBLY {
	#[serde(rename = "id")]
	pub id: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "ns")]
	pub ns: Option<URIDatatype>,
	#[serde(rename = "class")]
	pub class: Option<String>,
	#[serde(rename = "blockElementGroup")]
	pub block_element_group: Vec<BlockElementGroup>,
	#[serde(rename = "title")]
	pub title: Title,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "part")]
	pub part: Vec<OscalcontrolcommonpartASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
}


// Label is : A short, placeholder name for the parameter, which can be used as a substitute for a value if no value is assigned.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Label {
	#[serde(flatten)]
	pub markup_line_datatype: MarkupLineDatatype,
}


// Usage ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Usage {
	#[serde(flatten)]
	pub markup_multiline_datatype: MarkupMultilineDatatype,
}


// Remarks ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Remarks {
	#[serde(rename = "blockElementGroup")]
	pub block_element_group: Vec<BlockElementGroup>,
}


// OscalcontrolcommonparameterASSEMBLY is : Parameters provide a mechanism for the dynamic assignment of value(s) in a control.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcontrolcommonparameterASSEMBLY {
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "class")]
	pub class: Option<String>,
	#[serde(rename = "depends-on")]
	pub dependson: Option<String>,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "label")]
	pub label: Label,
	#[serde(rename = "usage")]
	pub usage: Usage,
	#[serde(rename = "constraint")]
	pub constraint: Vec<OscalcontrolcommonparameterconstraintASSEMBLY>,
	#[serde(rename = "guideline")]
	pub guideline: Vec<OscalcontrolcommonparameterguidelineASSEMBLY>,
	#[serde(rename = "value")]
	pub value: Vec<String>,
	#[serde(rename = "select")]
	pub select: OscalcontrolcommonparameterselectionASSEMBLY,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// Description is : A textual summary of the constraint to be applied.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Description {
	#[serde(flatten)]
	pub markup_multiline_datatype: MarkupMultilineDatatype,
}


// Test ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Test {
	#[serde(rename = "expression")]
	pub expression: Expression,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// OscalcontrolcommonparameterconstraintASSEMBLY is : A formal or informal expression of a constraint or test.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcontrolcommonparameterconstraintASSEMBLY {
	#[serde(rename = "description")]
	pub description: Description,
	#[serde(rename = "test")]
	pub test: Vec<Test>,
}


// OscalcontrolcommonparameterguidelineASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcontrolcommonparameterguidelineASSEMBLY {
	#[serde(rename = "blockElementGroup")]
	pub block_element_group: Vec<BlockElementGroup>,
}


// OscalcontrolcommonparametervalueFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcontrolcommonparametervalueFIELD {
	#[serde(rename = "oscal-control-common-parameter-value-FIELD")]
	pub oscalcontrolcommonparametervalue_field: String,
}


// Choice ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Choice {
	#[serde(flatten)]
	pub markup_line_datatype: MarkupLineDatatype,
}


// OscalcontrolcommonparameterselectionASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcontrolcommonparameterselectionASSEMBLY {
	#[serde(rename = "how-many")]
	pub howmany: Option<String>,
	#[serde(rename = "choice")]
	pub choice: Vec<Choice>,
}


// OscalcontrolcommonincludeallASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalcontrolcommonincludeallASSEMBLY {
}


// Revision ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Revision {
	#[serde(rename = "title")]
	pub title: Title,
	#[serde(rename = "published")]
	pub published: DateTimeWithTimezoneDatatype,
	#[serde(rename = "last-modified")]
	pub lastmodified: DateTimeWithTimezoneDatatype,
	#[serde(rename = "version")]
	pub version: String,
	#[serde(rename = "oscal-version")]
	pub oscalversion: String,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// Revisions ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Revisions {
	#[serde(rename = "revision")]
	pub revision: Vec<Revision>,
}


// Role ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Role {
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "title")]
	pub title: Title,
	#[serde(rename = "short-name")]
	pub shortname: Shortname,
	#[serde(rename = "description")]
	pub description: Description,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// URIDatatype ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct URIDatatype {
	#[serde(rename = "URIDatatype")]
	pub uri_datatype: URIDatatype,
}


// Location ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Location {
	#[serde(rename = "uuid")]
	pub uuid: UUIDDatatype,
	#[serde(rename = "title")]
	pub title: Title,
	#[serde(rename = "address")]
	pub address: OscalmetadataaddressASSEMBLY,
	#[serde(rename = "email-address")]
	pub emailaddress: Vec<EmailAddressDatatype>,
	#[serde(rename = "telephone-number")]
	pub telephonenumber: Vec<OscalmetadatatelephonenumberFIELD>,
	#[serde(rename = "url")]
	pub url: Vec<Url>,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// Externalid ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Externalid {
	#[serde(rename = "scheme")]
	pub scheme: URIDatatype,
	#[serde(flatten)]
	pub string_datatype: String,
}


// UUIDDatatype ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UUIDDatatype {
	#[serde(rename = "UUIDDatatype")]
	pub uuid_datatype: UUIDDatatype,
}


// Party ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Party {
	#[serde(rename = "uuid")]
	pub uuid: UUIDDatatype,
	#[serde(rename = "type")]
	pub type_attr: String,
	#[serde(rename = "name")]
	pub name: Name,
	#[serde(rename = "short-name")]
	pub shortname: Shortname,
	#[serde(rename = "external-id")]
	pub externalid: Vec<Externalid>,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "email-address")]
	pub emailaddress: Vec<EmailAddressDatatype>,
	#[serde(rename = "telephone-number")]
	pub telephonenumber: Vec<OscalmetadatatelephonenumberFIELD>,
	#[serde(rename = "address")]
	pub address: Vec<OscalmetadataaddressASSEMBLY>,
	#[serde(rename = "location-uuid")]
	pub locationuuid: Vec<UUIDDatatype>,
	#[serde(rename = "member-of-organization")]
	pub memberoforganization: Vec<Memberoforganization>,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// OscalmetadatametadataASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatametadataASSEMBLY {
	#[serde(rename = "title")]
	pub title: Title,
	#[serde(rename = "published")]
	pub published: DateTimeWithTimezoneDatatype,
	#[serde(rename = "last-modified")]
	pub lastmodified: DateTimeWithTimezoneDatatype,
	#[serde(rename = "version")]
	pub version: String,
	#[serde(rename = "oscal-version")]
	pub oscalversion: String,
	#[serde(rename = "revisions")]
	pub revisions: Revisions,
	#[serde(rename = "document-id")]
	pub documentid: Vec<OscalmetadatadocumentidFIELD>,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "role")]
	pub role: Vec<Role>,
	#[serde(rename = "location")]
	pub location: Vec<Location>,
	#[serde(rename = "party")]
	pub party: Vec<Party>,
	#[serde(rename = "responsible-party")]
	pub responsibleparty: Vec<OscalmetadataresponsiblepartyASSEMBLY>,
	#[serde(rename = "action")]
	pub action: Vec<OscalmetadataactionASSEMBLY>,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// OscalmetadatalocationuuidFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatalocationuuidFIELD {
	#[serde(rename = "oscal-metadata-location-uuid-FIELD")]
	pub oscalmetadatalocationuuid_field: UUIDDatatype,
}


// OscalmetadatapartyuuidFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatapartyuuidFIELD {
	#[serde(rename = "oscal-metadata-party-uuid-FIELD")]
	pub oscalmetadatapartyuuid_field: UUIDDatatype,
}


// OscalmetadataroleidFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataroleidFIELD {
	#[serde(rename = "oscal-metadata-role-id-FIELD")]
	pub oscalmetadataroleid_field: String,
}


// Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Text {
	#[serde(flatten)]
	pub markup_line_datatype: MarkupLineDatatype,
}


// Citation ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Citation {
	#[serde(rename = "text")]
	pub text: Text,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
}


// Rlink ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Rlink {
	#[serde(rename = "href")]
	pub href: String,
	#[serde(rename = "media-type")]
	pub mediatype: Option<String>,
	#[serde(rename = "hash")]
	pub hash: Vec<OscalmetadatahashFIELD>,
}


// Base64 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Base64 {
	#[serde(rename = "filename")]
	pub filename: Option<String>,
	#[serde(rename = "media-type")]
	pub mediatype: Option<String>,
	#[serde(flatten)]
	pub string: String,
}


// Resource ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Resource {
	#[serde(rename = "uuid")]
	pub uuid: UUIDDatatype,
	#[serde(rename = "title")]
	pub title: Title,
	#[serde(rename = "description")]
	pub description: Description,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "document-id")]
	pub documentid: Vec<OscalmetadatadocumentidFIELD>,
	#[serde(rename = "citation")]
	pub citation: Citation,
	#[serde(rename = "rlink")]
	pub rlink: Vec<Rlink>,
	#[serde(rename = "base64")]
	pub base64: Base64,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// OscalmetadatabackmatterASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatabackmatterASSEMBLY {
	#[serde(rename = "resource")]
	pub resource: Vec<Resource>,
}


// OscalmetadatapropertyASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatapropertyASSEMBLY {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "uuid")]
	pub uuid: Option<UUIDDatatype>,
	#[serde(rename = "ns")]
	pub ns: Option<URIDatatype>,
	#[serde(rename = "value")]
	pub value: String,
	#[serde(rename = "class")]
	pub class: Option<String>,
	#[serde(rename = "group")]
	pub group: Option<String>,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// OscalmetadatalinkASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatalinkASSEMBLY {
	#[serde(rename = "href")]
	pub href: String,
	#[serde(rename = "rel")]
	pub rel: Option<String>,
	#[serde(rename = "media-type")]
	pub mediatype: Option<String>,
	#[serde(rename = "resource-fragment")]
	pub resourcefragment: Option<String>,
	#[serde(rename = "text")]
	pub text: Text,
}


// OscalmetadataresponsiblepartyASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataresponsiblepartyASSEMBLY {
	#[serde(rename = "role-id")]
	pub roleid: String,
	#[serde(rename = "party-uuid")]
	pub partyuuid: Vec<UUIDDatatype>,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// OscalmetadataactionASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataactionASSEMBLY {
	#[serde(rename = "uuid")]
	pub uuid: UUIDDatatype,
	#[serde(rename = "date")]
	pub date: Option<u8>,
	#[serde(rename = "type")]
	pub type_attr: String,
	#[serde(rename = "system")]
	pub system: URIDatatype,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "responsible-party")]
	pub responsibleparty: Vec<OscalmetadataresponsiblepartyASSEMBLY>,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// OscalmetadataresponsibleroleASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataresponsibleroleASSEMBLY {
	#[serde(rename = "role-id")]
	pub roleid: String,
	#[serde(rename = "prop")]
	pub prop: Vec<OscalmetadatapropertyASSEMBLY>,
	#[serde(rename = "link")]
	pub link: Vec<OscalmetadatalinkASSEMBLY>,
	#[serde(rename = "party-uuid")]
	pub partyuuid: Vec<UUIDDatatype>,
	#[serde(rename = "remarks")]
	pub remarks: Remarks,
}


// OscalmetadatahashFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatahashFIELD {
	#[serde(rename = "algorithm")]
	pub algorithm: String,
	#[serde(flatten)]
	pub string_datatype: String,
}


// OscalmetadataremarksFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataremarksFIELD {
	#[serde(flatten)]
	pub markup_multiline_datatype: MarkupMultilineDatatype,
}


// OscalmetadatapublishedFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatapublishedFIELD {
	#[serde(rename = "oscal-metadata-published-FIELD")]
	pub oscalmetadatapublished_field: u8,
}


// OscalmetadatalastmodifiedFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatalastmodifiedFIELD {
	#[serde(rename = "oscal-metadata-last-modified-FIELD")]
	pub oscalmetadatalastmodified_field: u8,
}


// OscalmetadataversionFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataversionFIELD {
	#[serde(rename = "oscal-metadata-version-FIELD")]
	pub oscalmetadataversion_field: String,
}


// OscalmetadataoscalversionFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataoscalversionFIELD {
	#[serde(rename = "oscal-metadata-oscal-version-FIELD")]
	pub oscalmetadataoscalversion_field: String,
}


// OscalmetadataemailaddressFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataemailaddressFIELD {
	#[serde(rename = "oscal-metadata-email-address-FIELD")]
	pub oscalmetadataemailaddress_field: String,
}


// OscalmetadatatelephonenumberFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatatelephonenumberFIELD {
	#[serde(rename = "type")]
	pub type_attr: Option<String>,
	#[serde(flatten)]
	pub string_datatype: String,
}


// OscalmetadataaddressASSEMBLY ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataaddressASSEMBLY {
	#[serde(rename = "addr-line")]
	pub addrline: Vec<String>,
	#[serde(rename = "city")]
	pub city: City,
}


// city is : City, town or geographical region for the mailing address.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct City {
	#[serde(rename = "city")]
	pub city: City,
}


// state is : State, province or analogous geographical region for a mailing address.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct State {
	#[serde(rename = "state")]
	pub state: State,
}


// postalcode is : Postal or ZIP code for mailing address.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Postalcode {
	#[serde(rename = "postal-code")]
	pub postalcode: Postalcode,
}


// country is : Qualifies the kind of document identifier using a URI. If the scheme is not provided the value of the element will be interpreted as a string of characters.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Country {
	#[serde(rename = "country")]
	pub country: Country,
}


// type_attr ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Type_attr {
	#[serde(rename = "type")]
	pub type_attr: String,
}


// OscalmetadataaddrlineFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadataaddrlineFIELD {
	#[serde(rename = "oscal-metadata-addr-line-FIELD")]
	pub oscalmetadataaddrline_field: String,
}


// OscalmetadatadocumentidFIELD ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OscalmetadatadocumentidFIELD {
	#[serde(rename = "scheme")]
	pub scheme: Option<URIDatatype>,
	#[serde(flatten)]
	pub string_datatype: String,
}


// MarkupMultilineDatatype ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MarkupMultilineDatatype {
	#[serde(rename = "blockElementGroup")]
	pub block_element_group: Vec<BlockElementGroup>,
}


// memberoforganization is : A postal address for the location.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct memberoforganization {
	#[serde(rename = "member-of-organization")]
	pub memberoforganization: Vec<Memberoforganization>,
}


// shortname is : Indicates the type of external identifier.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Shortname {
	#[serde(rename = "short-name")]
	pub shortname: Shortname,
}


// name is : The full name of the party. This is typically the legal name associated with the party.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Name {
	#[serde(rename = "name")]
	pub name: Name,
}


// HeadingBlockElementGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HeadingBlockElementGroup {
	#[serde(rename = "h1")]
	pub h1: InlineMarkupType,
	#[serde(rename = "h2")]
	pub h2: InlineMarkupType,
	#[serde(rename = "h3")]
	pub h3: InlineMarkupType,
	#[serde(rename = "h4")]
	pub h4: InlineMarkupType,
	#[serde(rename = "h5")]
	pub h5: InlineMarkupType,
	#[serde(rename = "h6")]
	pub h6: InlineMarkupType,
}


// url is : An organization or person, which may be associated with roles or other concepts within the current or linked OSCAL document.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Url {
	#[serde(rename = "url")]
	pub url: Vec<Url>,
}


// expression is : A name given to the role, which may be used by a tool for display and navigation.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Expression {
	#[serde(rename = "expression")]
	pub expression: Expression,
}


// BlockElementGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BlockElementGroup {
	#[serde(rename = "p")]
	pub p: InlineMarkupType,
	#[serde(rename = "table")]
	pub table: TableType,
	#[serde(rename = "img")]
	pub img: ImageType,
	#[serde(rename = "headingBlockElementGroup")]
	pub heading_block_element_group: HeadingBlockElementGroup,
	#[serde(rename = "listsGroup")]
	pub lists_group: ListsGroup,
	#[serde(rename = "blockTextGroup")]
	pub block_text_group: BlockTextGroup,
}


// hr ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Hr {
	#[serde(rename = "hr")]
	pub hr: Hr,
}


// BlockTextGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BlockTextGroup {
	#[serde(rename = "pre")]
	pub pre: PreformattedType,
	#[serde(rename = "hr")]
	pub hr: Hr,
	#[serde(rename = "blockquote")]
	pub blockquote: BlockQuoteType,
}


// PreformattedType is The content model is the same as inlineMarkupType, but line endings need
//             to be preserved, since this is preformatted.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PreformattedType {
	#[serde(flatten)]
	pub inline_markup_type: InlineMarkupType,
}


// ListType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ListType {
	#[serde(rename = "li")]
	pub li: Vec<ListItemType>,
}


// OrderedListType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderedListType {
	#[serde(rename = "start")]
	pub start: Option<u32>,
	#[serde(flatten)]
	pub list_type: ListType,
}


// ListsGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ListsGroup {
	#[serde(rename = "ul")]
	pub ul: ListType,
	#[serde(rename = "ol")]
	pub ol: OrderedListType,
}


// ListItemType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ListItemType {
	#[serde(rename = "inlineMarkupGroup")]
	pub inline_markup_group: Vec<InlineMarkupGroup>,
	#[serde(rename = "listsGroup")]
	pub lists_group: Vec<ListsGroup>,
	#[serde(rename = "blockTextGroup")]
	pub block_text_group: Vec<BlockTextGroup>,
	#[serde(rename = "headingBlockElementGroup")]
	pub heading_block_element_group: Vec<HeadingBlockElementGroup>,
	#[serde(rename = "p")]
	pub p: Vec<InlineMarkupType>,
}


// TableType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TableType {
	#[serde(rename = "tr")]
	pub tr: Vec<TableRowType>,
}


// TableRowType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TableRowType {
	#[serde(rename = "td")]
	pub td: Vec<TableCellType>,
	#[serde(rename = "th")]
	pub th: Vec<TableCellType>,
}


// TableCellType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TableCellType {
	#[serde(rename = "align")]
	pub align: Option<String>,
	#[serde(flatten)]
	pub inline_markup_type: InlineMarkupType,
}


// AlignType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AlignType {
	#[serde(rename = "alignType")]
	pub align_type: String,
}


// BlockQuoteType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BlockQuoteType {
	#[serde(rename = "blockElementGroup")]
	pub block_element_group: Vec<BlockElementGroup>,
}


// MarkupLineDatatype ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MarkupLineDatatype {
	#[serde(flatten)]
	pub inline_markup_type: InlineMarkupType,
}


// InlineMarkupType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct InlineMarkupType {
	#[serde(rename = "inlineMarkupGroup")]
	pub inline_markup_group: Vec<InlineMarkupGroup>,
}


// br ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Br {
	#[serde(rename = "br")]
	pub br: Br,
}


// InlineMarkupGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct InlineMarkupGroup {
	#[serde(rename = "a")]
	pub a: AnchorType,
	#[serde(rename = "insert")]
	pub insert: InsertType,
	#[serde(rename = "br")]
	pub br: Br,
	#[serde(rename = "phraseMarkupGroup")]
	pub phrase_markup_group: PhraseMarkupGroup,
}


// PhraseMarkupGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PhraseMarkupGroup {
	#[serde(rename = "code")]
	pub code: CodeType,
	#[serde(rename = "em")]
	pub em: InlineMarkupType,
	#[serde(rename = "i")]
	pub i: InlineMarkupType,
	#[serde(rename = "b")]
	pub b: InlineMarkupType,
	#[serde(rename = "strong")]
	pub strong: InlineMarkupType,
	#[serde(rename = "sub")]
	pub sub: InlineMarkupType,
	#[serde(rename = "sup")]
	pub sup: InlineMarkupType,
	#[serde(rename = "q")]
	pub q: InlineMarkupType,
	#[serde(rename = "img")]
	pub img: ImageType,
}


// CodeType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CodeType {
	#[serde(rename = "class")]
	pub class: Option<String>,
	#[serde(flatten)]
	pub inline_markup_type: InlineMarkupType,
}


// ImageType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ImageType {
	#[serde(rename = "alt")]
	pub alt: Option<String>,
	#[serde(rename = "src")]
	pub src: String,
	#[serde(rename = "title")]
	pub title: Option<String>,
}


// AnchorType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AnchorType {
	#[serde(rename = "href")]
	pub href: Option<String>,
	#[serde(rename = "title")]
	pub title: Option<String>,
	#[serde(rename = "phraseMarkupGroup")]
	pub phrase_markup_group: Vec<PhraseMarkupGroup>,
}


// InsertType is . This insert mechanism allows the selection of which text value from the object to dynamically include based on the application's display requirements.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct InsertType {
	#[serde(rename = "type")]
	pub type_attr: String,
	#[serde(rename = "id-ref")]
	pub idref: String,
}


// Base64Datatype is Binary data encoded using the Base64 encoding algorithm
// as defined by RFC4648.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Base64Datatype {
	#[serde(rename = "Base64Datatype")]
	pub base64_datatype: String,
}


// DateTimeDatatype is A string representing a point in time with an optional timezone.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DateTimeDatatype {
	#[serde(rename = "DateTimeDatatype")]
	pub date_time_datatype: u8,
}


// DateTimeWithTimezoneDatatype is A string representing a point in time with a required timezone.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DateTimeWithTimezoneDatatype {
	#[serde(rename = "DateTimeWithTimezoneDatatype")]
	pub date_time_with_timezone_datatype: u8,
}


// EmailAddressDatatype is An email address string formatted according to RFC 6531.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EmailAddressDatatype {
	#[serde(rename = "EmailAddressDatatype")]
	pub email_address_datatype: String,
}


// TokenDatatype is A single token may not contain whitespace.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TokenDatatype {
	#[serde(rename = "TokenDatatype")]
	pub token_datatype: String,
}


// URIReferenceDatatype is This pattern ensures that leading and trailing whitespace is
// disallowed. This helps to even the user experience between implementations
// related to whitespace.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct URIReferenceDatatype {
	#[serde(rename = "URIReferenceDatatype")]
	pub uri_reference_datatype: String,
}
