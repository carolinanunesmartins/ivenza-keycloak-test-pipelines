<#--  using our own base layout as the style  -->
<#import "skantrae-template.ftl" as layout>
<#import "./components/input.ftl" as input>
<#import "./components/button.ftl" as button>
<#import "./components/card.ftl" as card>
<#import "./components/heading.ftl" as heading>
<#import "./components/text.ftl" as text>

<@layout.registrationLayout displayMessage=!messagesPerField.existsError('password','password-confirm'); section>
    <#if section = "header">
        ${msg("updatePasswordTitle")}
    <#elseif section = "form">
        <form id="kc-passwd-update-form" class="${properties.kcFormClass!}" action="${url.loginAction}" method="post" autocomplete="off">
            <@card.card>
                <@heading.h1>
                    ${kcSanitize(msg("kcUpdatePassTitle"))!"Title"}
                </@heading.h1>
                <@text.p>
                    ${kcSanitize(msg("kcUpdatePassDescription"))!"Password explanation"}
                </@text.p>
                <#--  passwords  -->
                <div class="flex flex-col gap-y-2 pt-2 pb-3">
                    <#--  new password  -->
                    <@input.passwordInput id="password-new" placeholder="Wachtwoord" />
                    <#--  confirm password  -->
                    <@input.passwordInput id="password-confirm" placeholder="Herhaal wachtwoord" />
                </div>

                <#--  submit button  -->
                <@button.secondaryButton type="submit">
                    ${kcSanitize(msg("kcUpdatePassConfirm"))!"Confirm"}
                </@button.secondaryButton>
            </@card.card>
        </form>
        <#--  allow the password visibility to be toggled  -->
        <script type="module" src="${url.resourcesPath}/js/passwordVisibility.js"></script>
    </#if>
</@layout.registrationLayout>