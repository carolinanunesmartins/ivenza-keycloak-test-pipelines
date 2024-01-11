<#--  The page you see when the update-password flow succeeds  -->

<#--  using our own base layout as the style  -->
<#import "skantrae-template.ftl" as layout>
<#import "./components/card.ftl" as card>
<#import "./components/heading.ftl" as heading>
<#import "./components/text.ftl" as text>

<@layout.registrationLayout displayMessage=false; section>
    <#if section = "header">
        <#if messageHeader??>
        ${messageHeader}
        <#else>
        ${message.summary}
        </#if>
    <#elseif section = "form">
    <div id="kc-info-message">
        <@card.card>
            <@heading.h1>
                ${kcSanitize(msg("kcUpdatePassTitle"))!"Title"}
            </@heading.h1>
            <@text.p>
                ${kcSanitize(msg("kcInfoDescription"))!"Info explanation"}
            </@text.p>
            <div class="flex flex-row justify-end">
                <@button.primaryButton type="button" id="redirectButton">
                    <div class="flex flex-row gap-2">
                        ${kcSanitize(msg("kcInfoLink"))!"Link"}
                        <span class="material-symbols-outlined">arrow_right_alt</span>
                    </div>
                </@button.primaryButton>
            </div>
        </@card.card>
    </div>
    <#--  allow the password visibility to be toggled  -->
    <script type="module" src="${url.resourcesPath}/js/redirect.js"></script>
    </#if>
</@layout.registrationLayout>
