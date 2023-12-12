<#--  The page you see when any error occurs  -->

<#--  using our own base layout as the style  -->
<#import "skantrae-template.ftl" as layout>
<#import "./components/card.ftl" as card>
<#import "./components/heading.ftl" as heading>
<#import "./components/text.ftl" as text>

<@layout.registrationLayout displayMessage=false; section>
    <#if section = "header">
        ${kcSanitize(msg("errorTitle"))?no_esc}
    <#elseif section = "form">
        <div id="kc-error-message">
            <@card.card>
                <@heading.h1>
                    ${kcSanitize(msg("kcErrorTitle"))!"Title"}
                </@heading.h1>
                <@text.p>
                    ${kcSanitize(msg("kcErrorDescription"))!"Error explanation"}
                </@text.p>
                <div class="flex flex-row justify-end">
                    <@text.rightArrowLink href="https://expert.skantrae.com/">
                        ${kcSanitize(msg("kcErrorLink"))!"Link"}
                    </@text.rightArrowLink>
                </div>
            </@card.card>
        </div>
    </#if>
</@layout.registrationLayout>