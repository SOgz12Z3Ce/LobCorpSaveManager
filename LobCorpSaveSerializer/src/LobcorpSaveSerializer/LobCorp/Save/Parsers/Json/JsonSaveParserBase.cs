using System;
using System.Collections.Generic;
using System.Linq;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public abstract class JsonSaveParserBase
	{
		private readonly SaveType type;
		public JsonSaveParserBase(SaveType type)
		{
			this.type = type;
		}
		public bool TryParse(JObject save, out Dictionary<string, object> data)
		{
			if (!CanParse(save))
			{
				data = null;
				return false;
			}
			try
			{
				data = Parse(save);
			}
			catch (NotImplementedException)
			{
				data = null;
				return false;
			}
			return true;
		}
		protected bool CanParse(JObject save)
		{
			return SaveTypeInferer.Infer(save) == type;
		}
		protected abstract Dictionary<string, object> Parse(JObject save);
		protected void Copy<T>(Dictionary<string, object> dest, JObject src, string prop)
		{
			dest[prop] = src[prop].Value<T>();
		}
		protected bool TryCopy<T>(Dictionary<string, object> dest, JObject src, string prop)
		{
			JToken value;
			if (!src.TryGetValue(prop, out value))
			{
				return false;
			}
			dest[prop] = value.Value<T>();
			return true;
		}
		protected void CopyList<T>(Dictionary<string, object> dest, JObject src, string prop)
		{
			dest[prop] = src[prop].Values<T>().ToList();
		}
	}
}
