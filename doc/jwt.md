# JWT

## Header

```json
{
  "alg": "SHA256",
  "typ": "JWT"
}
```

## Payload

The second part of the token is the payload, which contains the claims.
Claims are statements about an entity which is typically a user and additional data.
There are three types of claims:
 - [Registered](#registered-claims)
 - [Public](#public-claims)
 - [Private](#private-claims)

### Registered Claims

These are predefined claims which are *not mandatory* but are recommended.
These include `iss` (issuer), `exp` (expiration time), `sub` (subject), and `aud` (audience).
- **iss:** A case-sensitive string or URI that uniquely identifies the party that issued the JWT. 
Its interpretation is application specific (there is no central authority managing issuers).
- **sub:** A case-sensitive string or URI that uniquely identifies the party that this JWT carries information about. 
In other words, the claims contained in this JWT are statements about this party.
The JWT spec specifies that this claim must be unique in the context of the issuer or, in cases where that is not possible, globally unique.
Handling of this claim is application specific.
- **aud:** Either a single case-sensitive string or URI or an array of such values that uniquely identify the intended recipients of this JWT.
In other words, when this claim is present, the party reading the data in this JWT must find itself in the aud claim or disregard the data contained in the JWT.
As in the case of the `iss` and `sub` claims, this claim is application specific.
- **exp:** A number representing a specific date and time in the format "seconds since epoch" as defined by POSIX.
This claim sets the exact moment in which the JWT is considered invalid.
Some implementations may allow for a certain skew between clocks.
- **nfp**: from *not before* (time).
The opposite of the `exp` claim.
This claim sets the exact moment from which this JWT is considered valid.
- **iat**: from *issued at* (time).
- **jti**: from *JWT ID*.
A string representing a unique identifier for this JWT.
This claim may be used to differentiate JWTs with other similar content (preventing replays, for instance).
It is up to the implementation to guarantee uniqueness.

## Public Claims

Public claims are claims that are either registered with the 
[IANA JSON Web Token Claims registry](https://datatracker.ietf.org/doc/html/rfc7519#section-10.1) or named using a 
collision resistant name.
A complete list of public claims can be found at [iana.org](https://www.iana.org/assignments/jwt/jwt.xhtml).
Some of these include:
- **name:** Full name
- **profile:** Profile page URL
- **picture:** Profile picture URL
- **email_verified:** True if the e-mail address has been verified; otherwise false
- **zoneinfo:** Time zone
- **locale:** Locale
- **phone_number_verified:** True if the phone number has been verified; otherwise false
- **updated_at:** Time the information was last updated
- **azp:** Authorized party - the party to which the ID Token was issued
- **nonce:** Value used to associate a Client session with an ID Token (MAY also be used for nonce values in other applications of JWTs)
- **auth_time:** Time when the authentication occurred
- **at_hash:** Access Token hash value
- **c_hash:** Code hash value
- **sub_jwk:** Public key used to check the signature of an ID Token
- **cnf:** Confirmation
- **toe:** Time of Event
- **txn:** Transaction Identifier
- **rph:** Resource Priority Header Authorization
- **sid:** Session ID
- **scope:** Scope Values
- **client_id:** Client Identifier
- **roles:** Roles
- **groups:** Groups
- **entitlements:** Entitlements
- **token_introspection:** Token introspection response
- **ueid:** The Universal Entity ID
- **oemid:** Hardware OEM ID
- **sig_val_claims:** Signature Validation Token
- **authorization_details:** The claim authorization_details contains a JSON array of JSON objects representing the rights of the access token. Each JSON object contains the data to specify the authorization requirements for a certain type of resource.
- **verified_claims:** This container Claim is composed of the verification evidence related to a certain verification process and the corresponding Claims about the End-User which were verified in this process.
- **htm:** The HTTP method of the request
- **htu:** The HTTP URI of the request (without query and fragment parts)
- **ath:** The base64url-encoded SHA-256 hash of the ASCII encoding of the associated access token's value
- **atc:** Authority Token Challenge
- **sub_id:** Subject Identifier
- **_claim_names:** JSON object whose member names are the Claim Names for the Aggregated and Distributed Claims
- **_claim_sources:** JSON object whose member names are referenced by the member values of the _claim_names member

## Private Claims

Private claims are those that are defined by *users* (consumers and producers) of the JWTs.
These are ad hoc claims for a particular case.