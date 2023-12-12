<#macro primaryButton disabled="false" type="button">
    <@button color="primary" disabled=disabled type=type>
        <#nested />
    </@button>
</#macro>

<#macro secondaryButton disable="false" type="button">
    <@button color="secondary" disabled=disabled type=type>
        <#nested />
    </@button>
</#macro>

<#--  compared to the main app, the following elements have been left out from the button: leading, trailing, truncated, loading  -->
<#macro button color disabled="false" type="button">
    <#if color == "primary">
        <#if disabled == "true">
            <button class="
                flex flex-row items-center justify-center relative cursor-not-allowed 
                bg-brand-primary-500 border border-brand-primary-500 text-white opacity-50 
                px-4 py-3 text-base font-bold tracking-wide leading-tight rounded-lg"
                disabled
                type="${type}"
            >
                <#nested />
            </button>
        <#else>
            <button class="
                flex flex-row items-center justify-center relative 
                bg-brand-primary-500 border border-brand-primary-500 text-white hover:bg-brand-primary-600 hover:border-brand-primary-600 
                px-4 py-3 text-base font-bold tracking-wide leading-tight rounded-lg"
                type="${type}"
            >
                <#nested />
            </button>
        </#if>
    <#elseif color == "secondary">
        <#if disabled == "true">
            <button class="
                flex flex-row items-center justify-center relative cursor-not-allowed 
                bg-brand-secondary-500 border border-brand-secondary-500 text-white opacity-50 
                px-4 py-3 text-base font-bold tracking-wide leading-tight rounded-lg"
                disabled
                type="${type}"
            >
                <#nested />
            </button>
        <#else>
            <button class="
                flex flex-row items-center justify-center relative 
                bg-brand-secondary-500 border border-brand-secondary-500 text-white hover:bg-brand-secondary-600 hover:border-brand-secondary-600 
                px-4 py-3 text-base font-bold tracking-wide leading-tight rounded-lg"
                type="${type}"
            >
                <#nested />
            </button>
        </#if>
    </#if>
</#macro>