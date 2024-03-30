(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[405],{1464:function(e,s,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/",function(){return n(3301)}])},3301:function(e,s,n){"use strict";n.r(s),n.d(s,{__toc:function(){return l},default:function(){return d}});var t=n(5893),a=n(2673),r=n(373);n(7294);let i={logo:(0,t.jsxs)("span",{style:{display:"flex",gap:"0.5rem"},children:[(0,t.jsx)("img",{src:"/faking/_next/static/media/LogoBackground.7a3e6887.png",width:"24px",alt:"",style:{borderRadius:"4px"}})," DataConstruct Docs"]}),head:(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)("link",{rel:"icon",href:"/assets/Logo.ico",sizes:"any"}),(0,t.jsx)("link",{rel:"icon",href:"/assets/Logo.svg",type:"image/svg+xml"})]}),project:{link:"https://github.com/Data-Construct/faking"},editLink:{text:"Edit this page on GitHub"},docsRepositoryBase:"https://github.com/Data-Construct/faking/tree/main/docs",primaryHue:219,chat:{link:"https://discord.gg/zxcpKF4db7"},footer:{text:(0,t.jsx)(t.Fragment,{children:(0,t.jsx)("a",{href:"https://www.dataconstruct.io",children:"DataConstruct.io"})})}};n(9128);var o=n(2643);let l=[{depth:2,value:"Features",id:"features"},{depth:2,value:"Usage Rust",id:"usage-rust"},{depth:2,value:"Usage Javascript / Typescript",id:"usage-javascript--typescript"},{depth:2,value:"Randomness seed",id:"randomness-seed"},{depth:3,value:"Unsupported Seeded Generation",id:"unsupported-seeded-generation"}];function _createMdxContent(e){let s=Object.assign({h1:"h1",p:"p",a:"a",img:"img",h2:"h2",ul:"ul",li:"li",blockquote:"blockquote",pre:"pre",code:"code",span:"span",h3:"h3"},(0,o.a)(),e.components);return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(s.h1,{children:"Getting Started"}),"\n",(0,t.jsx)(s.p,{children:"Generate massive amounts of fake (but realistic) data for testing and development."}),"\n",(0,t.jsxs)("div",{style:{marginTop:"16px",display:"flex",gap:"8px"},children:[(0,t.jsx)("span",{children:(0,t.jsx)(s.a,{href:"https://docs.rs/data-faking",children:(0,t.jsx)(s.img,{src:"https://docs.rs/data-faking/badge.svg",alt:"Docs Status"})})}),(0,t.jsx)("span",{children:(0,t.jsx)(s.a,{href:"https://crates.io/crates/data-faking",children:(0,t.jsx)(s.img,{src:"https://img.shields.io/crates/v/data-faking.svg",alt:"Latest Version"})})}),(0,t.jsx)("span",{children:(0,t.jsx)(s.a,{href:"https://www.npmjs.com/package/data-faking",children:(0,t.jsx)(s.img,{src:"https://badgen.net/npm/v/data-faking",alt:"npm version"})})})]}),"\n",(0,t.jsxs)(s.p,{children:["Try using our ",(0,t.jsx)(s.a,{href:"https://www.dataconstruct.io/organizations/playground/schemas",children:"playground"})," for your data gen needs, it supports code gen for much more than rust and javascript."]}),"\n",(0,t.jsx)(s.h2,{id:"features",children:"Features"}),"\n",(0,t.jsxs)(s.ul,{children:["\n",(0,t.jsx)(s.li,{children:"Defaults data types - numbers, lorem ipsum, bools, uuids"}),"\n",(0,t.jsx)(s.li,{children:"People - generate names, emails, jobs"}),"\n",(0,t.jsx)(s.li,{children:"Locations - generate addresses for north america (more coming soon), and coordinates"}),"\n",(0,t.jsx)(s.li,{children:"Various media - games, show, and books from across the globe"}),"\n",(0,t.jsx)(s.li,{children:"API data - generate data resembling real apis (ex. stripe)"}),"\n"]}),"\n",(0,t.jsxs)(s.blockquote,{children:["\n",(0,t.jsx)(s.p,{children:"Note: We try to generate realistic data. The generated names, addresses, emails, phone numbers, and/or other data might be coincidentally valid information. Please do not send any of your messages / calls to them from your test setup."}),"\n"]}),"\n",(0,t.jsx)(s.h2,{id:"usage-rust",children:"Usage Rust"}),"\n",(0,t.jsx)(s.pre,{"data-language":"bash","data-theme":"default",children:(0,t.jsx)(s.code,{"data-language":"bash","data-theme":"default",children:(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:"cargo"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-string)"},children:"add"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-string)"},children:"data-faking"})]})})}),"\n",(0,t.jsx)(s.pre,{"data-language":"rust","data-theme":"default",children:(0,t.jsxs)(s.code,{"data-language":"rust","data-theme":"default",children:[(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"use"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" data_faking "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"as"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" faking;"})]}),"\n",(0,t.jsx)(s.span,{className:"line",children:" "}),"\n",(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"fn"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:"main"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"() {"})]}),"\n",(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"  "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:"println!"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"("}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-string-expression)"},children:'"{}"'}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:", faking"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"::"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"defaults"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"::"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"types"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"::"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:"f64"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"());"})]}),"\n",(0,t.jsx)(s.span,{className:"line",children:(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"}"})})]})}),"\n",(0,t.jsx)(s.h2,{id:"usage-javascript--typescript",children:"Usage Javascript / Typescript"}),"\n",(0,t.jsx)(s.pre,{"data-language":"bash","data-theme":"default",children:(0,t.jsx)(s.code,{"data-language":"bash","data-theme":"default",children:(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:"npm"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-string)"},children:"i"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-string)"},children:"--save-dev"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-string)"},children:"data-faking"})]})})}),"\n",(0,t.jsx)(s.pre,{"data-language":"typescript","data-theme":"default",children:(0,t.jsxs)(s.code,{"data-language":"typescript","data-theme":"default",children:[(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"import"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-constant)"},children:"*"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"as"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" faking "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"from"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:" "}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-string-expression)"},children:'"data-faking"'}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:";"})]}),"\n",(0,t.jsx)(s.span,{className:"line",children:" "}),"\n",(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-constant)"},children:"console"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:".log"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"("}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-constant)"},children:"faking"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:".f64"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"());"})]})]})}),"\n",(0,t.jsx)(s.h2,{id:"randomness-seed",children:"Randomness seed"}),"\n",(0,t.jsx)(s.p,{children:"If you want consistent results, you can set your own seed:"}),"\n",(0,t.jsx)(s.pre,{"data-language":"rust","data-theme":"default",children:(0,t.jsxs)(s.code,{"data-language":"rust","data-theme":"default",children:[(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"faking"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"::"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"utils"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"::"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"seeder"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"::"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:"set_seed"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"("}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-constant)"},children:"2"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:");"})]}),"\n",(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:"println!"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"("}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-string-expression)"},children:'"{}"'}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:", faking"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"::"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"defaults"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"::"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"types"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-keyword)"},children:"::"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:"f64"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"());"})]})]})}),"\n",(0,t.jsx)(s.pre,{"data-language":"typescript","data-theme":"default",children:(0,t.jsxs)(s.code,{"data-language":"typescript","data-theme":"default",children:[(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-constant)"},children:"faking"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:".set_seed"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"("}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:"BigInt"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"("}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-constant)"},children:"2"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"));"})]}),"\n",(0,t.jsxs)(s.span,{className:"line",children:[(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-constant)"},children:"console"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:".log"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"("}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-constant)"},children:"faking"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-token-function)"},children:".f64"}),(0,t.jsx)(s.span,{style:{color:"var(--shiki-color-text)"},children:"());"})]})]})}),"\n",(0,t.jsx)(s.h3,{id:"unsupported-seeded-generation",children:"Unsupported Seeded Generation"}),"\n",(0,t.jsx)(s.p,{children:"The following data generators do not support seeded generation (currently):"}),"\n",(0,t.jsxs)(s.ul,{children:["\n",(0,t.jsx)(s.li,{children:"UUIDs, any."}),"\n",(0,t.jsx)(s.li,{children:"DateTime: Naive Date (Before Today | After Today)."}),"\n"]})]})}let c={MDXContent:function(){let e=arguments.length>0&&void 0!==arguments[0]?arguments[0]:{},{wrapper:s}=Object.assign({},(0,o.a)(),e.components);return s?(0,t.jsx)(s,{...e,children:(0,t.jsx)(_createMdxContent,{...e})}):_createMdxContent(e)},pageOpts:{filePath:"pages/index.mdx",route:"/",timestamp:171181309e4,pageMap:[{kind:"Meta",data:{index:"Guide",contact:{title:"Contact ↗",type:"page",href:"https://twitter.com/dataconstructio",newWindow:!0}}},{kind:"MdxPage",name:"index",route:"/"}],flexsearch:{codeblocks:!0},title:"Getting Started",headings:l},pageNextRoute:"/",nextraLayout:r.ZP,themeConfig:i};var d=(0,a.j)(c)},5789:function(){}},function(e){e.O(0,[774,796,888,179],function(){return e(e.s=1464)}),_N_E=e.O()}]);