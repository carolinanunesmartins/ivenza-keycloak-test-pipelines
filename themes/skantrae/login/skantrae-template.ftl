<#import "./components/logo.ftl" as logo>
<#import "./components/text.ftl" as text>

<#macro registrationLayout displayMessage=true returnLink="https://expert.skantrae.com/">
    <!DOCTYPE html>
    <html class=""<#if realm.internationalizationEnabled> lang="${locale.currentLanguageTag}"</#if>>
        <head>
            <meta charset="utf-8">
            <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
            <meta name="robots" content="noindex, nofollow">

            <#--  import the material symbols font  -->
            <link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:FILL@0..1&display=block"
                />

            <#if properties.meta?has_content>
                <#list properties.meta?split(' ') as meta>
                    <meta name="${meta?split('==')[0]}" content="${meta?split('==')[1]}"/>
                </#list>
            </#if>
            <title>${msg("loginTitle",(realm.displayName!''))}</title>
            <link rel="icon" href="${url.resourcesPath}/img/favicon.ico" />
            <#if properties.stylesCommon?has_content>
                <#list properties.stylesCommon?split(' ') as style>
                    <link href="${url.resourcesCommonPath}/${style}" rel="stylesheet" />
                </#list>
            </#if>
            <#if properties.styles?has_content>
                <#list properties.styles?split(' ') as style>
                    <link href="${url.resourcesPath}/${style}" rel="stylesheet" />
                </#list>
            </#if>
            <#if properties.scripts?has_content>
                <#list properties.scripts?split(' ') as script>
                    <script src="${url.resourcesPath}/${script}" type="text/javascript"></script>
                </#list>
            </#if>
            <#if scripts??>
                <#list scripts as script>
                    <script src="${script}" type="text/javascript"></script>
                </#list>
            </#if>
            <#if authenticationSession??>
                <script type="module">
                    import { checkCookiesAndSetTimer } from "${url.resourcesPath}/js/authChecker.js";

                    checkCookiesAndSetTimer(
                    "${authenticationSession.authSessionId}",
                    "${authenticationSession.tabId}",
                    "${url.ssoLoginInOtherTabsUrl}"
                    );
                </script>
            </#if>
        </head>

        <body class="bg-brand-gray-50">
            <#--  Header  -->
            <span class="flex flex-row items-stretch py-2 bg-brand-secondary-500 text-white h-16 relative z-50 flex flex-1 flex-row justify-between items-center">
                <div class="7xl:px-0 mx-auto flex w-full max-w-[1440px] flex-col items-stretch px-2.5 md:px-10 justify-center">
                    <div class="flex flex-row justify-between items-center">
                        <#--  header back link  -->
                        <a href="${returnLink}" class="flex flex-row gap-4 items-center hover:text-white hover:no-underline">
                            <span class="material-symbols-outlined">arrow_left_alt</span>
                            <@text.p>
                                ${kcSanitize(msg("kcTemplateBack"))!"Terug"}
                            </@text.p>
                        </button>

                        <#--  header logo  -->
                        <a href="https://expert.skantrae.com/" class="contents">
                            <@logo.skantraeLogo />
                        </a>
                    </div>
                </div>
            </span>
            <#--  main content  -->
            <div class="7xl:px-0 mx-auto flex w-full max-w-[1440px] flex-col items-stretch px-2.5 md:px-10 justify-center">
                <div class="h-screen flex flex-row justify-center items-center -m-20">
                    <#nested "form">
                </div>
            </div>
        </body>
    </html>
</#macro>
