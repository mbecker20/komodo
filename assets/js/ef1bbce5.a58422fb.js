"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[100],{8151:(e,n,i)=>{i.r(n),i.d(n,{assets:()=>d,contentTitle:()=>o,default:()=>u,frontMatter:()=>s,metadata:()=>a,toc:()=>c});var t=i(4848),r=i(8453);const s={},o="Builders",a={id:"build-images/builders",title:"Builders",description:"A builder is a machine running the Komodo Periphery agent (and usually docker), which is able to handle a RunBuild / BuildRepo command from Komodo core. Any server connected to Komodo can be chosen as the builder for a build.",source:"@site/docs/build-images/builders.md",sourceDirName:"build-images",slug:"/build-images/builders",permalink:"/docs/build-images/builders",draft:!1,unlisted:!1,editUrl:"https://github.com/mbecker20/komodo/tree/main/docsite/docs/build-images/builders.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Pre-build command",permalink:"/docs/build-images/pre-build"},next:{title:"Image Versioning",permalink:"/docs/build-images/versioning"}},d={},c=[{value:"AWS builder",id:"aws-builder",level:2},{value:"Setup the instance",id:"setup-the-instance",level:3},{value:"Make an AMI from the instance",id:"make-an-ami-from-the-instance",level:3},{value:"Configure security groups / firewall",id:"configure-security-groups--firewall",level:3}];function l(e){const n={admonition:"admonition",code:"code",h1:"h1",h2:"h2",h3:"h3",header:"header",p:"p",pre:"pre",...(0,r.R)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.header,{children:(0,t.jsx)(n.h1,{id:"builders",children:"Builders"})}),"\n",(0,t.jsx)(n.p,{children:"A builder is a machine running the Komodo Periphery agent (and usually docker), which is able to handle a RunBuild / BuildRepo command from Komodo core. Any server connected to Komodo can be chosen as the builder for a build."}),"\n",(0,t.jsx)(n.p,{children:"Building on a machine running production software is usually not a great idea, as this process can use a lot of system resources. It is better to start up a temporary cloud machine dedicated for the build, then shut it down when the build is finished. Komodo supports AWS EC2 for this task."}),"\n",(0,t.jsx)(n.h2,{id:"aws-builder",children:"AWS builder"}),"\n",(0,t.jsx)(n.p,{children:"Builders are now Komodo resources, and are managed via the core API / can be updated using the UI.\nTo use this feature, you need an AWS EC2 AMI with docker and Komodo Periphery configured to run on system start.\nOnce you create your builder and add the necessary configuration, it will be available to attach to builds."}),"\n",(0,t.jsx)(n.h3,{id:"setup-the-instance",children:"Setup the instance"}),"\n",(0,t.jsx)(n.p,{children:"Create an EC2 instance, and install Docker and Periphery on it."}),"\n",(0,t.jsx)(n.p,{children:"The following script is an example of installing Docker and Periphery onto a Ubuntu/Debian instance:"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-sh",children:"#!/bin/bash\napt update\napt upgrade -y\ncurl -fsSL https://get.docker.com | sh\nsystemctl enable docker.service\nsystemctl enable containerd.service\ncurl -sSL https://raw.githubusercontent.com/mbecker20/komodo/main/scripts/setup-periphery.py | HOME=/root python3\nsystemctl enable periphery.service\n"})}),"\n",(0,t.jsx)(n.admonition,{type:"note",children:(0,t.jsx)(n.p,{children:'AWS provides a "user data" feature, which will run a provided script as root. The above can be used with AWS user data\nto provide a hands free setup.'})}),"\n",(0,t.jsx)(n.h3,{id:"make-an-ami-from-the-instance",children:"Make an AMI from the instance"}),"\n",(0,t.jsx)(n.p,{children:"Once the instance is up and running, ssh in and confirm Periphery is running using:"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-sh",children:"sudo systemctl status periphery.service\n"})}),"\n",(0,t.jsx)(n.p,{children:"If it is not, the install hasn't finished and you should wait a bit. It may take 5 minutes or more (all in updating / installing Docker, Periphery is just a 12 MB binary to download)."}),"\n",(0,t.jsxs)(n.p,{children:["Once Periphery is running, you can navigate to the instance in the AWS UI and choose ",(0,t.jsx)(n.code,{children:"Actions"})," -> ",(0,t.jsx)(n.code,{children:"Image and templates"})," -> ",(0,t.jsx)(n.code,{children:"Create image"}),". Just name the image and hit create."]}),"\n",(0,t.jsxs)(n.p,{children:["The AMI will provide a unique id starting with ",(0,t.jsx)(n.code,{children:"ami-"}),", use this with the builder configuration."]}),"\n",(0,t.jsx)(n.h3,{id:"configure-security-groups--firewall",children:"Configure security groups / firewall"}),"\n",(0,t.jsx)(n.p,{children:"The builders will need inbound access on port 8120 from Komodo Core, be sure to add a security group with this rule to the Builder configuration."})]})}function u(e={}){const{wrapper:n}={...(0,r.R)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(l,{...e})}):l(e)}},8453:(e,n,i)=>{i.d(n,{R:()=>o,x:()=>a});var t=i(6540);const r={},s=t.createContext(r);function o(e){const n=t.useContext(s);return t.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function a(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:o(e.components),t.createElement(s.Provider,{value:n},e.children)}}}]);