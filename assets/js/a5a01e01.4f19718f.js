"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[549],{6175:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>a,default:()=>u,frontMatter:()=>s,metadata:()=>r,toc:()=>d});var i=n(4848),o=n(8453);const s={},a="Image Versioning",r={id:"build-images/versioning",title:"Image Versioning",description:"Komodo uses a major.minor.patch versioning scheme to Build versioning. By default, every RunBuild will auto increment the Build's version patch number, and push the image to docker hub with the version tag, as well as the latest tag. A tag containing the latest short commit hash at the time the repo was cloned will also be created.",source:"@site/docs/build-images/versioning.md",sourceDirName:"build-images",slug:"/build-images/versioning",permalink:"/docs/build-images/versioning",draft:!1,unlisted:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/build-images/versioning.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Builders",permalink:"/docs/build-images/builders"},next:{title:"Deploy Containers",permalink:"/docs/deploy-containers/"}},c={},d=[];function l(e){const t={code:"code",h1:"h1",header:"header",p:"p",...(0,o.R)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(t.header,{children:(0,i.jsx)(t.h1,{id:"image-versioning",children:"Image Versioning"})}),"\n",(0,i.jsxs)(t.p,{children:["Komodo uses a major.minor.patch versioning scheme to Build versioning. By default, every RunBuild will auto increment the Build's version patch number, and push the image to docker hub with the version tag, as well as the ",(0,i.jsx)(t.code,{children:"latest"})," tag. A tag containing the latest short commit hash at the time the repo was cloned will also be created."]}),"\n",(0,i.jsxs)(t.p,{children:['You can also turn off the auto incrementing feature, and manage the version yourself. In addition, you can configure a "version tag" on the build. This will postfix the version tag / commit hash tag with a custom label. For example, an image tag of ',(0,i.jsx)(t.code,{children:"dev"})," will produce tags like ",(0,i.jsx)(t.code,{children:"image_name:1.1.1-dev"})," and ",(0,i.jsx)(t.code,{children:"image_name:h3c87c-dev"}),"."]})]})}function u(e={}){const{wrapper:t}={...(0,o.R)(),...e.components};return t?(0,i.jsx)(t,{...e,children:(0,i.jsx)(l,{...e})}):l(e)}},8453:(e,t,n)=>{n.d(t,{R:()=>a,x:()=>r});var i=n(6540);const o={},s=i.createContext(o);function a(e){const t=i.useContext(s);return i.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function r(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(o):e.components||o:a(e.components),i.createElement(s.Provider,{value:t},e.children)}}}]);