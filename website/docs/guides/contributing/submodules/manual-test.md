# Manual tests

Though we have [a lot of tests and CI](../../miscellaneous/safety),
there are rare cases when manual tests serve as a complement.
This page documents these.

## Demo (Gallery)

Though we have automatic integration tests, here we discuss manual extra tests.

### Testing environments

Try to test on as many browsers as possible, e.g.

* Chrome
* Firefox
* Safari

### Testing procedure

* Prepare cold-start environment (see below)
* Open documentation page "demo" and play with it
* Leave and re-enter the page (s.t. non-cold-start environment), and play with it again

### Appendix: Prepare cold-start environment

* Remove service workers: `navigator.serviceWorker.getRegistrations().then(function(registrations) { for(let registration of registrations) { registration.unregister(); }});`
  * Chrome/Firefox: `F12` - paste the code - enter
* Do "clear browser cache"
  * Chrome: `F12` - `Network` - right-click - Clear Cache
  * Firefox: Settings - Privacy&Security - Cookies and Site Data - Clear Data - Clear
  * Safari: Develop (menu) - Empty cache
