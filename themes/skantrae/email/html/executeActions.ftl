<#outputformat "plainText">
<#assign requiredActionsText><#if requiredActions??><#list requiredActions><#items as reqActionItem>${msg("requiredAction.${reqActionItem}")}<#sep>, </#sep></#items></#list></#if></#assign>
</#outputformat>
<#import "template.ftl" as layout>
<#import "../components/header.ftl" as header>
<#import "../components/footer.ftl" as footer>
<@layout.emailLayout>
<@header.header/>
<table
    cellpadding="0"
    cellspacing="0"
    align="center"
    style="padding: 30px; font-family: Arial; width: 730px; background-color: #ffffff"
>
    <tbody>
        <tr>
            <td height="100%" valign="top">
                <table
                    style="width: 100%; border-spacing: 0; border-collapse: collapse"
                    cellpadding="0"
                    cellspacing="0"
                    align="left"
                >
                    <tbody>
                        <tr>
                            <td style="padding: 0; margin: 0; border-spacing: 0">
                                <table cellpadding="0" cellspacing="0" style="table-layout: fixed; width: 100%">
                                    <tbody>
                                        <tr>
                                            <td
                                                style="padding: 18px 10px 18px 0; text-align: inherit"
                                                height="100%"
                                                valign="top"
                                            >
                                                <div>
                                                    <div>
                                                        ${kcSanitize(msg("executeActionsBodyHtml",link, linkExpiration, realmName, requiredActionsText, linkExpirationFormatter(linkExpiration)))?no_esc}
                                                    </div>
                                                </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </td>
        </tr>
    </tbody>
</table>
<@footer.footer/>
</@layout.emailLayout>
