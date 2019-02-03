pub fn text() -> String {
r#"
<h1 id="routing">Routing</h1>
<p>Seed includes flexible routing, inspired by <a href="https://github.com/reasonml/reason-react/blob/master/docs/router.md">React-Reason</a>: You can trigger state changes that update the address bar, and can be nagivated to/from using forward and back buttons. This works for landing-page routing as well, provided your server is configured to support. For an examples, see the <a href="https://github.com/David-OConnor/seed/tree/master/examples/homepage">homepage</a> or <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">todomvc</a> examples.</p>
<p>Let's say our site the following pages: a guide, which can have subpages, and a changelog, accessible by <code>http://seed-rs.org/changelog</code>, <code>http://seed-rs.org/guide</code>, and <code>http://seed-rs.org/guide/3' (where 3 is the page we want) respectively.  We describe the page by a</code>page` field in our model, which is an integer: 0 for guide, 1 for changelog, and an additional number for the guide page. (An enum would be cleaner, but we don't wish to complicate this example).</p>
<p>To set up the initial routing, we pass a <code>Routes</code> function describing how to handle routing, to <code>App::build</code>'s <code>routes</code> method.(https://docs.rs/seed/0.2.5/seed/fn.run.html).</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="kw">fn</span> routes(url: <span class="pp">seed::</span>Url) -&gt; Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb1-2" title="2">    <span class="kw">if</span> url.path.len() == <span class="dv">0</span> <span class="op">{</span></a>
<a class="sourceLine" id="cb1-3" title="3">        <span class="kw">return</span> <span class="pp">Msg::</span>ChangeVisibility(<span class="pp">Visible::</span>All)</a>
<a class="sourceLine" id="cb1-4" title="4">    <span class="op">}</span></a>
<a class="sourceLine" id="cb1-5" title="5"></a>
<a class="sourceLine" id="cb1-6" title="6">    <span class="kw">match</span> url.path<span class="op">[</span><span class="dv">0</span><span class="op">]</span>.as_ref() <span class="op">{</span></a>
<a class="sourceLine" id="cb1-7" title="7">        <span class="st">&quot;guide&quot;</span> =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb1-8" title="8">            <span class="co">// Determine if we&#39;re at the main guide page, or a subpage</span></a>
<a class="sourceLine" id="cb1-9" title="9">            <span class="kw">match</span> url.path.get(<span class="dv">1</span>).as_ref() <span class="op">{</span></a>
<a class="sourceLine" id="cb1-10" title="10">                <span class="cn">Some</span>(page) =&gt; <span class="pp">Msg::</span>ChangeGuidePage(page.<span class="pp">parse::</span>&lt;<span class="dt">usize</span>&gt;().unwrap()),</a>
<a class="sourceLine" id="cb1-11" title="11">                <span class="cn">None</span> =&gt; <span class="pp">Msg::</span>ChangePage(<span class="dv">0</span>)</a>
<a class="sourceLine" id="cb1-12" title="12">            <span class="op">}</span></a>
<a class="sourceLine" id="cb1-13" title="13">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb1-14" title="14">        <span class="st">&quot;changelog&quot;</span> =&gt; <span class="pp">Msg::</span>ChangePage(<span class="dv">1</span>),</a>
<a class="sourceLine" id="cb1-15" title="15">        _ =&gt; <span class="pp">Msg::</span>ChangePage(<span class="dv">0</span>),</a>
<a class="sourceLine" id="cb1-16" title="16">    <span class="op">}</span></a>
<a class="sourceLine" id="cb1-17" title="17"><span class="op">}</span></a>
<a class="sourceLine" id="cb1-18" title="18"></a>
<a class="sourceLine" id="cb1-19" title="19"><span class="at">#[</span>wasm_bindgen<span class="at">]</span></a>
<a class="sourceLine" id="cb1-20" title="20"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb1-21" title="21">    <span class="pp">seed::App::</span>build(<span class="pp">Model::</span><span class="kw">default</span>(), update, view)</a>
<a class="sourceLine" id="cb1-22" title="22">        .routes(routes)</a>
<a class="sourceLine" id="cb1-23" title="23">        .finish()</a>
<a class="sourceLine" id="cb1-24" title="24">        .run();</a>
<a class="sourceLine" id="cb1-25" title="25"><span class="op">}</span></a></code></pre></div>
<p>the <a href="https://docs.rs/seed/0.2.5/seed/routing/struct.Url.html">Url struct</a> which routes has the following fields, which describe the route:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="kw">pub</span> <span class="kw">struct</span> Url <span class="op">{</span></a>
<a class="sourceLine" id="cb2-2" title="2">    <span class="kw">pub</span> path: <span class="dt">Vec</span>&lt;<span class="dt">String</span>&gt;,</a>
<a class="sourceLine" id="cb2-3" title="3">    <span class="kw">pub</span> hash: <span class="dt">Option</span>&lt;<span class="dt">String</span>&gt;,</a>
<a class="sourceLine" id="cb2-4" title="4">    <span class="kw">pub</span> search: <span class="dt">Option</span>&lt;<span class="dt">String</span>&gt;,</a>
<a class="sourceLine" id="cb2-5" title="5">    <span class="kw">pub</span> title: <span class="dt">Option</span>&lt;<span class="dt">String</span>&gt;,</a>
<a class="sourceLine" id="cb2-6" title="6"><span class="op">}</span></a></code></pre></div>
<p><code>path</code> contains the path heirarchy from top to bottom. For example, the <code>changelog</code> page above's path is <code>vec![String::from("changelog")]</code>, representing <code>/changelog/</code>, and guide page 3's is <code>vec![String::from("guide"), 3.to_string()]</code>, representing <code>/guide/3/</code>. The other three properties aren't as common; <code>hash</code> describes text after a <code>#</code>; <code>search</code> describes text after a <code>?</code>, but before <code>#</code>, and title is unimplemented in current web browsers, but may find use in the future.</p>
<p>To make landing-page routing work, configure your server so that all three of these paths point towards the app, or that any (sub)path points towards it, instead of returning an error. The <code>serve.py</code> script included in the quickstart repo and examples is set up for this. Once this is configured, intial routing on page load will work as expected: The page will load with the default state, then immediately trigger the update prescribed by the RoutePage message.</p>
<p>In order to trigger our route change through in-app naviation (eg clicking a link or pushing a button), include logic like this in the update function:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb3-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb3-3" title="3">    RoutePage(<span class="dt">u32</span>),</a>
<a class="sourceLine" id="cb3-4" title="4">    RouteGuidePage(<span class="dt">u32</span>),</a>
<a class="sourceLine" id="cb3-5" title="5">    ChangePage(<span class="dt">u32</span>),</a>
<a class="sourceLine" id="cb3-6" title="6">    ChangeGuidePage(<span class="dt">u32</span>),</a>
<a class="sourceLine" id="cb3-7" title="7"><span class="op">}</span></a>
<a class="sourceLine" id="cb3-8" title="8"></a>
<a class="sourceLine" id="cb3-9" title="9"><span class="kw">fn</span> update(msg: Msg, model: Model) -&gt; Model <span class="op">{</span></a>
<a class="sourceLine" id="cb3-10" title="10">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb3-11" title="11">        <span class="pp">Msg::</span>RoutePage(page) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-12" title="12">            <span class="pp">seed::</span>push_route(</a>
<a class="sourceLine" id="cb3-13" title="13">                <span class="pp">seed::Url::</span>new(<span class="pp">vec!</span><span class="op">[</span>&amp;page.to_string()<span class="op">]</span>)</a>
<a class="sourceLine" id="cb3-14" title="14">            );</a>
<a class="sourceLine" id="cb3-15" title="15">            update(<span class="pp">Msg::</span>ChangePage(page), model)</a>
<a class="sourceLine" id="cb3-16" title="16">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-17" title="17">        <span class="pp">Msg::</span>RouteGuidePage(guide_page) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb3-18" title="18">            <span class="pp">seed::</span>push_route(</a>
<a class="sourceLine" id="cb3-19" title="19">                <span class="pp">seed::Url::</span>new(<span class="pp">vec!</span><span class="op">[</span><span class="st">&quot;guide&quot;</span>, &amp;guide_page.to_string()<span class="op">]</span>)</a>
<a class="sourceLine" id="cb3-20" title="20">            );</a>
<a class="sourceLine" id="cb3-21" title="21">            update(<span class="pp">Msg::</span>ChangeGuidePage(guide_page), model)</a>
<a class="sourceLine" id="cb3-22" title="22">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb3-23" title="23">        <span class="co">// This is separate, because nagivating the route triggers state updates, which would</span></a>
<a class="sourceLine" id="cb3-24" title="24">        <span class="co">// trigger an additional push state.</span></a>
<a class="sourceLine" id="cb3-25" title="25">        <span class="pp">Msg::</span>ChangePage(page) =&gt; Render(Model <span class="op">{</span>page, ..model<span class="op">}</span>),</a>
<a class="sourceLine" id="cb3-26" title="26">        <span class="pp">Msg::</span>ChangeGuidePage(guide_page) =&gt; Render(Model <span class="op">{</span>guide_page, page: <span class="pp">Page::</span>Guide, ..model<span class="op">}</span>),</a>
<a class="sourceLine" id="cb3-27" title="27">    <span class="op">}</span></a>
<a class="sourceLine" id="cb3-28" title="28"><span class="op">}</span></a></code></pre></div>
<p>Notice how the <code>Route</code> messages above call <a href="https://docs.rs/seed/0.2.5/seed/fn.push_route.html">seed::push_route</a>, and the <code>Change</code> ones call the state, are called in the <code>routes</code> function, and are recursively called in the <code>Route</code> messages. <code>seed::Url::new</code> is a method for creating the <code>Url</code> struct above. It creates a new Url from a <code>Vec</code> of <code>&amp;strs</code>, converts them to the <code>Vec&lt;String&gt;</code> found in <code>Url</code>, and makes the rest of the fields <code>None</code>. If you wish to define one of these fields, there are additional methods you can chain together, eg: <code>seed::url::New(vec!["myurl"]).hash("textafterhash")</code></p>
<p><code>push_route</code> accepts a single parameter: a <code>Url</code> struct.</p>
<p>When a page is loaded or browser naviation occurs (eg back button), Seed uses the <code>routes</code> func you provided to determine what message to call.</p>
<p>Notice how we keep ChangePage and RoutePage separate in our example. Do not call <code>push_route</code> from one of these messages, or you'll end up with recusions/unwanted behavior: <code>ChangePage</code> in our example performs the action associated with routing, while <code>RoutePage</code> updates our route history, then recursively calls <code>ChangePage</code>. If you were to attempt this in the same message, each browser navigation event would add a redundant route history entry, interfering with navigation. `</p>
<p>We call <code>RoutePage</code> from an in-app navigation event, like this:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="pp">h2!</span><span class="op">[</span> simple_ev(<span class="pp">Ev::</span>Click, <span class="pp">Msg::</span>RoutePage), <span class="st">&quot;Guide&quot;</span> <span class="op">]</span></a></code></pre></div>
<p>Or can call it programatically using lifecycle hooks:</p>
<div class="sourceCode" id="cb5"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb5-1" title="1">    did_mount(<span class="kw">move</span> |_| <span class="op">{</span></a>
<a class="sourceLine" id="cb5-2" title="2">        <span class="kw">if</span> model.logged_in <span class="op">{</span></a>
<a class="sourceLine" id="cb5-3" title="3">            state.update(<span class="pp">Msg::</span>RoutePage(<span class="pp">Page::</span>Main))</a>
<a class="sourceLine" id="cb5-4" title="4">        <span class="op">}</span></a>
<a class="sourceLine" id="cb5-5" title="5">    <span class="op">}</span>)</a></code></pre></div>
"#.into()
}