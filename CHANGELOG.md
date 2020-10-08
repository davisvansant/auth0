## 0.3.0 (2020-10-8)
___

 - Cleanup and move initial `authentication` and `management` crates up under main `auth0` src
 - Add new steps in `.drone.yml` to break out individual tests
 - Bump to Rust `1.47.0`

## 0.2.0 (2020-10-5)
___

 - Initial implementation of authentication crate
 - Includes Auth0's Authentication API endpoints, using `reqwest` to build the HTTP client and prepare requests for sending
 - Includes unit and integration(mocks) tests for endpoints found here:
  - https://auth0.com/docs/api/authentication

## 0.1.0 (2020-08-25)
___

  - Initial Release
