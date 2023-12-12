<#import "./logo.ftl" as logo>
<#macro header>
<table cellpadding="0" cellspacing="0" align="center" style="padding: 30px; width: 100%; background-color: #024d9e">
    <tbody>
        <tr>
            <td valign="top" style="height: 100%">
                <table
                    style="width: 720px; border-spacing: 0; border-collapse: collapse; margin: 0 10px"
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
                                                style="font-size: 6px; line-height: 10px; padding: 0"
                                                valign="top"
                                                align="center"
                                            >
                                                <@logo.skantraeLogo />
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
</#macro>
