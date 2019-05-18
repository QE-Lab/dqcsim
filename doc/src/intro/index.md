<h1 style="text-align: center">DQCsim</h1>
<p style="text-align: center; padding-bottom: 10px; font-size: larger; font-weight: bold">
The Delft Quantum & Classical simulator
</p>
<p style="text-align: center; padding-bottom: 20px">
DQCsim is a <i>framework</i> that can be used to tie <i>components</i> of
quantum computer simulators together in a <i>standardized</i> yet
<i>extensible</i>, <i>developer-friendly</i>, and <i>reproducible</i> way.
</p>
<table style="border: solid 1px rgba(0.5,0.5,0.5,0.1); background-color: rgba(0.5,0.5,0.5,0.03)">
<tr>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; font-size: larger; padding-top: 20px; font-weight: bold">Framework</td>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; font-size: larger; padding-top: 20px; font-weight: bold">Components</td>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; font-size: larger; padding-top: 20px; font-weight: bold">Developer-friendly</td>
</tr>
<tr>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; vertical-align: top; padding-bottom: 20px; font-size: smaller">
DQCsim only provides <a href="../intro/interfaces.html">interfaces</a>
to tie simulator components together. That is, it does not contain any
simulation code on its own. DQCsim is all the boilerplate code that you don't
want to write when you're developing a new way to
<a href="../intro/backend.html">simulate qubits</a>, a
<a href="../intro/frontend.html">microarchitecture simulator</a>, an
<a href="../intro/operator.html">error model</>, etc.
</td>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; vertical-align: top; padding-bottom: 20px; font-size: smaller">
DQCsim abstracts a quantum computer simulation into four components:
<a href="../intro/host.html">hosts</a>,
<a href="../intro/frontend.html">frontends</a>,
<a href="../intro/operator.html">operators</a>, and
<a href="../intro/backend.html">backends</a>. These components are separate
operating system processes that each fulfill a well-defined function within the
simulation, thus splitting the simulation up into more manageable parts.
</td>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; vertical-align: top; padding-bottom: 20px; font-size: smaller">
All the components can be written in <a href="../python-api/index.html">Python</a>,
<a href="../c-api/index.html">C</a>, <a href="../cpp-api/index.html">C++</a>, or
<a href="../rust-api/index.html">Rust</a>. Just use whichever language you
prefer &ndash; or any combination of those languages! DQCsim will automatically
tie everything together for you at runtime, so you can focus on writing quantum
algorithms instead of fighting CPython.
</td>
</tr>
<tr>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; font-size: larger; padding-top: 20px; font-weight: bold">Standardized</td>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; font-size: larger; padding-top: 20px; font-weight: bold">Extensible</td>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; font-size: larger; padding-top: 20px; font-weight: bold">Reproducible</td>
</tr>
<tr>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; vertical-align: top; padding-bottom: 30px; font-size: smaller">
DQCsim fully specifies a set of core features that each component
<a href="../c-api/pdef.apigen.html#assigning-callback-functions">needs to
support</a>, as well as the interfaces used to drive them. Therefore, as long as the
components don't rely on any user-defined extra features in other components,
they can be swapped out without breaking anything.
</td>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; vertical-align: top; padding-bottom: 30px; font-size: smaller">
Besides standardizing the basic features of each component, DQCsim provides
<a href="arbs.html">an interface</a> for users to implement their own features,
without needing to change anything in DQCsim's codebase. So don't panic about
DQCsim being written in Rust: you shouldn't need to read or write a single line
of code in here!
</td>
<td style="border-style: none; background-color: rgba(0.5,0.5,0.5,0.03); text-align: center; vertical-align: top; padding-bottom: 30px; font-size: smaller">
While quantum mechanics are inherently stochastic, simulating it
<a href="../intro/reproducibility.html">needs not be</a>. DQCsim provides a
random generator to the components that should be more than random enough for
simulation purposes, while being reproducible when this is desirable, such as
while debugging.
</td>
</tr>
</table>
<h2 style="text-align: center">Interested?</h2>
<h1 style="text-align: center"><a href="../intro/components.html">Keep reading!</a></h1>
<h3 style="text-align: center">(<a href="../install/index.html">or skip directly to the install notes</a>)</h2>
