"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[782],{8327:(e,o,t)=>{t.r(o),t.d(o,{assets:()=>c,contentTitle:()=>s,default:()=>l,frontMatter:()=>i,metadata:()=>h,toc:()=>d});var r=t(4848),n=t(8453);const i={},s="Configuring Webhooks",h={id:"webhooks",title:"Configuring Webhooks",description:"Multiple Komodo resources can take advantage of webhooks from your git provider. Komodo supports incoming webhooks using the Github standard, which is also supported by other providers like Gitea.",source:"@site/docs/webhooks.md",sourceDirName:".",slug:"/webhooks",permalink:"/docs/webhooks",draft:!1,unlisted:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/webhooks.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Sync Resources",permalink:"/docs/sync-resources"},next:{title:"Permissioning Resources",permalink:"/docs/permissioning"}},c={},d=[{value:"Copy the Resource Payload URL",id:"copy-the-resource-payload-url",level:2},{value:"Create the webhook on the Git Provider",id:"create-the-webhook-on-the-git-provider",level:2},{value:"When does it trigger?",id:"when-does-it-trigger",level:2},{value:"Procedure webhooks",id:"procedure-webhooks",level:2}];function a(e){const o={admonition:"admonition",code:"code",em:"em",h1:"h1",h2:"h2",header:"header",li:"li",ol:"ol",p:"p",pre:"pre",strong:"strong",...(0,n.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(o.header,{children:(0,r.jsx)(o.h1,{id:"configuring-webhooks",children:"Configuring Webhooks"})}),"\n",(0,r.jsx)(o.p,{children:"Multiple Komodo resources can take advantage of webhooks from your git provider. Komodo supports incoming webhooks using the Github standard, which is also supported by other providers like Gitea."}),"\n",(0,r.jsx)(o.admonition,{type:"note",children:(0,r.jsx)(o.p,{children:'On Gitea, the default "Gitea" webhook type works with the Github standard \ud83d\udc4d'})}),"\n",(0,r.jsx)(o.h2,{id:"copy-the-resource-payload-url",children:"Copy the Resource Payload URL"}),"\n",(0,r.jsxs)(o.p,{children:["Find the resource in UI, like a ",(0,r.jsx)(o.code,{children:"Build"}),", ",(0,r.jsx)(o.code,{children:"Repo"}),", or ",(0,r.jsx)(o.code,{children:"Stack"}),".\nScroll down to the bottom of Configuration area, and copy the webhook for the action you want."]}),"\n",(0,r.jsx)(o.h2,{id:"create-the-webhook-on-the-git-provider",children:"Create the webhook on the Git Provider"}),"\n",(0,r.jsx)(o.p,{children:"Navigate to the repo page on your git provider, and go to the settings for the Repo.\nFind Webhook settings, and click to create a new webhook."}),"\n",(0,r.jsx)(o.p,{children:"You will have to input some information."}),"\n",(0,r.jsxs)(o.ol,{children:["\n",(0,r.jsxs)(o.li,{children:["The ",(0,r.jsx)(o.code,{children:"Payload URL"})," is the link that you copied in the step above, ",(0,r.jsx)(o.code,{children:"Copy the Resource Payload URL"}),"."]}),"\n",(0,r.jsxs)(o.li,{children:["For Content-type, choose ",(0,r.jsx)(o.code,{children:"application/json"})]}),"\n",(0,r.jsxs)(o.li,{children:["For Secret, input the secret you configured in the Komodo Core config (",(0,r.jsx)(o.code,{children:"KOMODO_WEBHOOK_SECRET"}),")."]}),"\n",(0,r.jsx)(o.li,{children:"Enable SSL Verification, if you have proper TLS setup to your git provider (recommended)."}),"\n",(0,r.jsx)(o.li,{children:'For "events that trigger the webhook", just the push request is what post people want.'}),"\n",(0,r.jsx)(o.li,{children:'Of course, make sure the webhook is "Active" and hit create.'}),"\n"]}),"\n",(0,r.jsx)(o.h2,{id:"when-does-it-trigger",children:"When does it trigger?"}),"\n",(0,r.jsxs)(o.p,{children:["Your git provider will now push this webhook to Komodo on ",(0,r.jsx)(o.em,{children:"every"})," push to ",(0,r.jsx)(o.em,{children:"any"})," branch. However, your ",(0,r.jsx)(o.code,{children:"Build"}),", ",(0,r.jsx)(o.code,{children:"Repo"}),",\netc. only cares about a specific branch of the repo."]}),"\n",(0,r.jsxs)(o.p,{children:["Because of this, the webhook will trigger the action ",(0,r.jsx)(o.strong,{children:"only on pushes to the branch configured on the resource"}),"."]}),"\n",(0,r.jsxs)(o.p,{children:["For example, if I make a build, I may point the build to the ",(0,r.jsx)(o.code,{children:"release"})," branch of a particular repo. If I set up a webhook, and push to the ",(0,r.jsx)(o.code,{children:"main"})," branch, the action will ",(0,r.jsx)(o.em,{children:"not trigger"}),". It will only trigger when the push is to the ",(0,r.jsx)(o.code,{children:"release"})," branch."]}),"\n",(0,r.jsx)(o.h2,{id:"procedure-webhooks",children:"Procedure webhooks"}),"\n",(0,r.jsx)(o.p,{children:"Not all actions support webhooks directly, however for those that don't, they can still be triggered via webhook by using a Procedure. Just create a Procedure and configure it to run the action you are looking for, and create a webhook pointing to the Procedure."}),"\n",(0,r.jsx)(o.p,{children:"Since Procedures don't specificy a particular branch it should listen for pushes on, this information\nmust be put in the webhook payload url. Procedures use webhook payload urls of the form:"}),"\n",(0,r.jsx)(o.pre,{children:(0,r.jsx)(o.code,{children:"<KOMODO_HOST>/listener/github/procedure/<PROCEDURE_ID>/<LISTEN_BRANCH>\n"})}),"\n",(0,r.jsxs)(o.p,{children:["If the ",(0,r.jsx)(o.code,{children:"<LISTEN_BRANCH>"})," is not provided, it will default to listening on the ",(0,r.jsx)(o.code,{children:"main"})," branch."]})]})}function l(e={}){const{wrapper:o}={...(0,n.R)(),...e.components};return o?(0,r.jsx)(o,{...e,children:(0,r.jsx)(a,{...e})}):a(e)}},8453:(e,o,t)=>{t.d(o,{R:()=>s,x:()=>h});var r=t(6540);const n={},i=r.createContext(n);function s(e){const o=r.useContext(i);return r.useMemo((function(){return"function"==typeof e?e(o):{...o,...e}}),[o,e])}function h(e){let o;return o=e.disableParentContext?"function"==typeof e.components?e.components(n):e.components||n:s(e.components),r.createElement(i.Provider,{value:o},e.children)}}}]);