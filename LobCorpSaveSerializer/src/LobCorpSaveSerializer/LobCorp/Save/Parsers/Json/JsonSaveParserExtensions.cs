using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public static class DictionaryExtensions
	{
		public static void CopyValue<T>(this Dictionary<string, object> dest, JObject src, string prop)
		{
			dest[prop] = src[prop].Value<T>();
		}
		public static bool TryCopyValue<T>(this Dictionary<string, object> dest, JObject src, string prop)
		{
			JToken value;
			if (!src.TryGetValue(prop, out value))
			{
				return false;
			}
			dest[prop] = value.Value<T>();
			return true;
		}
		public static void CopyValues<T>(this Dictionary<string, object> dest, JObject src, string prop)
		{
			dest[prop] = src[prop].Values<T>().ToList();
		}
	}
}
