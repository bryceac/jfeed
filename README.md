# jfeed

**Author:** Bryce Campbell

**License:** See LICENSE

**Description:** A crate for Rust that allows one to read and write JSON feeds.

**Version:** 0.1.0

## Notes

This crate is currently a <abbr title="work in progress">WIP</abbr>, and has not been fully implemented as intended.

### Questions

<ol>
<li>
<dt style="font-weight:bold">Why create this when there are other crates that can be used?</dt>
<dd>
<p>I would have very much liked to try out the ones listed <a href="https://www.jsonfeed.org/code/">here</a>, but they either appeared to be unmaintained, which
is not that surprising, since the spec hardly changes, only parses JSON feeds, or uses rather ugly code that reminds me of old Java code.</p>
<p>While I cannot promise my code is any prettier, it will at least do what I am writing this do to, which is create JSON feeds.</p>
</dd>
</li>
<li>
<dt style="font-weight:bold">What Specs does this crate support?</dt>
<dd>
<p>At this time, it only supports version 1.1 of the JSON feed spec.</p>
<p>However, it should be easy to add stuff for newer versions.</p>
</dd>
</li>
<li>
<dt style="font-weight:bold">I see that much of the way things are created are very similiar to Paul Woolcock&#39;s <a href="https://github.com/pwoolcoc/jsonfeed">jsonfeed</a> crate. Why didn&#39;t you just fork that?</dt>
<dd>
<p>Funny that you should ask that.</p>
<p>When I was originally looking for crates to deal with JSON feeds, 
his crate was actually the one that I wanted to use, as it used a patterns common to Rust, unlike <a href="https://crates.io/crates/json-feed-model">json-feed-model</a>, which uses code similiar to Java.</p>
<p>Sadly, Paul&#39;s crate appears to be unmaintained, and there is a pull request on it that was never merged.</p>
<p>However, my biggest reason for not forking his work is because it felt like I would have put in as much effort as I did with this because he did some things that I don&#39;t agree with with.</p>
<p>For example, the JSON feed spec notes down data types as they would be
found in Javascript, but then notes that some of them are actually URLs.</p>
<p>Paul follows this verbatim, while you would typically want a URL type in Swift for these instances.</p>
<p>as such, I just decideed to roll my own crate.</p>
</dd>
</li>
<li>
<dt style="font-weight:bold">I see that you have made some stuff required that isn&#39;t required by the JSON feed spec. Why?</dt>
<dd>
<p>This crate was made for the purpose of implementing a news feed and/or
blog component of a custom <abbr title="static site generator">SSG</abbr>,
and, to fit that purpose, things like dates and contentmare important.</p>
<p>If you need more flexibility, you&#39;ll either want to look for a different crate or fork this one and make things more flexible.</p>
</dd>
</li>
</ol>

### Contributing

If you would like to help make this crate better, feel free to fork this repo and make a pull request.

Please be aware that your contributions will be licensed under the same license as this crate.

