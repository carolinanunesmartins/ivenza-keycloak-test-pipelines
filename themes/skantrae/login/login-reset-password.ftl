<#--  using our own base layout as the style  -->
<#import "skantrae-template.ftl" as layout>
<#import "./components/input.ftl" as input>
<#import "./components/button.ftl" as button>
<#import "./components/card.ftl" as card>
<#import "./components/heading.ftl" as heading>
<#import "./components/text.ftl" as text>

<@layout.registrationLayout displayMessage=!messagesPerField.existsError('username') returnLink="https://expert.skantrae.com/account/login"; section>
    <#if section = "header">
        ${msg("emailForgotTitle")}
    <#elseif section = "form">
        <form id="kc-reset-password-form" class="${properties.kcFormClass!}" action="${url.loginAction}" method="post">
            <@card.card>
                <div class="flex flex-col justify-center items-center space-y-6 max-w-sm">
                    <@heading.h1>
                        ${kcSanitize(msg("emailForgotTitle"))}
                    </@heading.h1>
                    <@text.p>
                        ${kcSanitize(msg("forgotExplanation"))}
                    </@text.p>
                    <div class="flex flex-col gap-y-2 pt-2 pb-3 w-full">
                        <@input.textInput id="username" placeholder="email" errorMessageKey="username" />
                    </div>
                    <@button.primaryButton type="submit" class="w-full">
                        ${kcSanitize(msg("forgotButtonText"))!"doSubmit"}
                </@button.primaryButton>
                </div>
            </@card.card>
        </form>
    <#elseif section = "info" >
        <#if realm.duplicateEmailsAllowed>
            ${msg("emailInstructionUsername")}
        <#else>
            ${msg("emailInstruction")}
        </#if>
    </#if>
</@layout.registrationLayout>
