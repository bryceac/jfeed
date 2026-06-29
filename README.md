# jfeed

**Author:** Bryce Campbell

**License:** See LICENSE

**Description:** A crate for Rust that allows one to read and write JSON feeds.

**Version:** 0.1.0

## Notes

This crate is currently a <abbr title="work in progress">WIP</abbr>, and has not been fully implemented as intended.

### To Do

<ul>
<li>Finish implementing the necessary elements, as outlined in the [JSON Feed 1.1 spec](https://www.jsonfeed.org/version/1.1/)</l>
<li>Write tests for Items, dates, and content.</li>
<li>Write tests for Feed Generation.</p>
<li>Write tests regarding feed parsing.</p>
</ul>

### Questions

<ol>
<li>
<dt style="font-weight:bold">Why create this when there are other crates that can be used?</dt>
<dd>
<p>I would have very much liked to try out the ones listed [here](https://www.jsonfeed.org/code/), but they either appeared to be unmaintained, which
is not that surprising, since the spec hardly changes, only parses JSON feeds, or uses rather code that reminds me of old Java code.</p>
<p>While I cannot promise my code is any prettier, it will at least do what I am writing this do to, which is create JSON feeds.</p>
</dd>
</li>
</ol>

### Contributing

If you would like to help get this project running, especially by implementing the stuff in the to do list, feel free to fork this project and make a pull request.

Please be aware that your contributions will be licensed under the same license as this crate.

