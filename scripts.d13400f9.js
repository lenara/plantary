parcelRequire=function(e,r,t,n){var i,o="function"==typeof parcelRequire&&parcelRequire,u="function"==typeof require&&require;function f(t,n){if(!r[t]){if(!e[t]){var i="function"==typeof parcelRequire&&parcelRequire;if(!n&&i)return i(t,!0);if(o)return o(t,!0);if(u&&"string"==typeof t)return u(t);var c=new Error("Cannot find module '"+t+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[t][1][r]||r},p.cache={};var l=r[t]=new f.Module(t);e[t][0].call(l.exports,p,l,l.exports,this)}return r[t].exports;function p(e){return f(p.resolve(e))}}f.isParcelRequire=!0,f.Module=function(e){this.id=e,this.bundle=f,this.exports={}},f.modules=e,f.cache=r,f.parent=o,f.register=function(r,t){e[r]=[function(e,r){r.exports=t},{}]};for(var c=0;c<t.length;c++)try{f(t[c])}catch(e){i||(i=e)}if(t.length){var l=f(t[t.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=l:"function"==typeof define&&define.amd?define(function(){return l}):n&&(this[n]=l)}if(parcelRequire=f,i)throw i;return f}({"VJtr":[function(require,module,exports) {
!function(o){"use strict";o('a.js-scroll-trigger[href*="#"]:not([href="#"])').click(function(){if(location.pathname.replace(/^\//,"")==this.pathname.replace(/^\//,"")&&location.hostname==this.hostname){var a=o(this.hash);if((a=a.length?a:o("[name="+this.hash.slice(1)+"]")).length)return o("html, body").animate({scrollTop:a.offset().top-71},1e3,"easeInOutExpo"),!1}}),o(document).scroll(function(){o(this).scrollTop()>100?o(".scroll-to-top").fadeIn():o(".scroll-to-top").fadeOut()}),o(".js-scroll-trigger").click(function(){o(".navbar-collapse").collapse("hide")}),o("body").scrollspy({target:"#mainNav",offset:80});var a=function(){o("#mainNav").offset().top>100?o("#mainNav").addClass("navbar-shrink"):o("#mainNav").removeClass("navbar-shrink")};a(),o(window).scroll(a),o(function(){o("body").on("input propertychange",".floating-label-form-group",function(a){o(this).toggleClass("floating-label-form-group-with-value",!!o(a.target).val())}).on("focus",".floating-label-form-group",function(){o(this).addClass("floating-label-form-group-with-focus")}).on("blur",".floating-label-form-group",function(){o(this).removeClass("floating-label-form-group-with-focus")})})}(jQuery);
},{}]},{},["VJtr"], null)
//# sourceMappingURL=scripts.d13400f9.js.map