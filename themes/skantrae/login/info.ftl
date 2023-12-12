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
                <@text.rightArrowLink href="https://expert.skantrae.com/account/login">
                    ${kcSanitize(msg("kcInfoLink"))!"Link"}
                </@text.rightArrowLink>
            </div>
        </@card.card>
    </div>
    </#if>
</@layout.registrationLayout>