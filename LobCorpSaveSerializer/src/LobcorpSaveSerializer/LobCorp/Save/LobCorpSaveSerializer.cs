using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization.Formatters.Binary;
using LobCorp.Exceptions;

namespace LobCorp.Save
{
	public class LobCorpSaveSerializer
	{
		public static LobCorpSaveSegment Deserialize(FileStream stream)
		{
			BinaryFormatter binaryFormatter = new BinaryFormatter();
			Dictionary<string, object> save;
			try
			{
				save = binaryFormatter.Deserialize(stream) as Dictionary<string, object>;
			}
			catch (Exception e)
			{
				throw new BadSaveFile(string.Format("Unable to deserialize file: {0}", stream.Name), e);
			}
			return new LobCorpSaveSegment(save);
		}
	}
}
